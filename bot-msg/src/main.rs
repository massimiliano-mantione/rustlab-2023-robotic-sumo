use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    net::SocketAddr,
};

use chrono::{DateTime, Local};
use clap::{self, Parser};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream},
    spawn,
    sync::{mpsc, oneshot},
};
use ulid::Ulid;

const VERSION: &str = "1.0";
const AUTHOR: &str = "Massimiliano Mantione";

#[derive(Parser, Debug)]
#[clap(version = VERSION, author = AUTHOR)]
pub struct Arguments {
    /// Bot port
    #[clap(short, long, default_value = "9001")]
    pub bot_port: u16,
    /// Client port
    #[clap(short, long, default_value = "9002")]
    pub client_port: u16,
    /// The action that must be performed
    #[clap(subcommand)]
    pub action: SubCommand,
}

#[derive(Parser, Debug)]
pub struct BrokerArguments {
    /// Address
    #[clap(short, long, default_value = "0.0.0.0")]
    pub address: String,
}

#[derive(Parser, Debug)]
pub struct CmdArguments {
    /// Address
    #[clap(short, long, default_value = "127.0.0.1")]
    pub address: String,
    /// Bot name
    #[clap(short, long)]
    pub name: String,
    /// Also support referee commands
    #[clap(short, long)]
    pub referee: bool,
}

#[derive(Parser, Debug)]
pub struct RefereeArguments {
    /// Address
    #[clap(short, long, default_value = "127.0.0.1")]
    pub address: String,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Broker(BrokerArguments),
    Cmd(CmdArguments),
    Referee(RefereeArguments),
}

pub enum RefereeCommand {
    Start,
    Stop,
}

impl RefereeCommand {
    pub fn encode(&self) -> u8 {
        match self {
            RefereeCommand::Start => 'x' as u8,
            RefereeCommand::Stop => 'z' as u8,
        }
    }

    pub fn decode(byte: u8) -> Option<Self> {
        if byte == 'x' as u8 || byte == 'X' as u8 {
            Some(Self::Start)
        } else if byte == 'z' as u8 || byte == 'Z' as u8 {
            Some(Self::Stop)
        } else {
            None
        }
    }
}

impl std::fmt::Display for RefereeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "referee command {}",
            match self {
                RefereeCommand::Start => "START",
                RefereeCommand::Stop => "STOP",
            }
        ))
    }
}

pub struct PrivateCommand {
    byte: u8,
}

impl PrivateCommand {
    pub fn encode(&self) -> u8 {
        self.byte
    }

    pub fn decode(byte: u8) -> Option<Self> {
        if (byte >= 'a' as u8 && byte <= 'z' as u8)
            || (byte >= 'A' as u8 && byte <= 'Z' as u8)
            || (byte >= '0' as u8 && byte <= '9' as u8)
        {
            if let Some(_) = RefereeCommand::decode(byte) {
                None
            } else {
                Some(Self { byte })
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for PrivateCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("private command '{}'", self.byte as char))
    }
}

pub enum BotCommand {
    Referee(RefereeCommand),
    Private(PrivateCommand),
}

impl BotCommand {
    pub fn encode(&self) -> u8 {
        match self {
            BotCommand::Referee(c) => c.encode(),
            BotCommand::Private(c) => c.encode(),
        }
    }

    pub fn decode(byte: u8) -> Option<Self> {
        RefereeCommand::decode(byte)
            .map(BotCommand::Referee)
            .or_else(|| PrivateCommand::decode(byte).map(BotCommand::Private))
    }
}

pub type BrokerResult = Result<(), String>;
pub type BrokerResultSender = oneshot::Sender<BrokerResult>;
pub type BrokerResultReceiver = oneshot::Receiver<BrokerResult>;

pub enum BrokerAction {
    BotJoin {
        id: Ulid,
        address: SocketAddr,
        writer: OwnedWriteHalf,
        sender: BrokerResultSender,
    },
    BotNameClaim {
        id: Ulid,
        name: String,
        sender: BrokerResultSender,
    },
    Join {
        id: Ulid,
        address: SocketAddr,
        writer: OwnedWriteHalf,
        sender: BrokerResultSender,
    },
    NameClaim {
        id: Ulid,
        name: String,
        sender: BrokerResultSender,
    },
    RefereeClaim {
        id: Ulid,
        sender: BrokerResultSender,
    },
    Log {
        id: Ulid,
        time: DateTime<Local>,
        message: String,
    },
    RefereeCommand {
        id: Ulid,
        time: DateTime<Local>,
        command: RefereeCommand,
    },
    PrivateCommand {
        id: Ulid,
        time: DateTime<Local>,
        command: PrivateCommand,
    },
    BotLeave {
        id: Ulid,
    },
    Leave {
        id: Ulid,
    },
}

pub type BrokerActionSender = mpsc::Sender<BrokerAction>;
pub type BrokerActionReceiver = mpsc::Receiver<BrokerAction>;

pub struct BrokerBot {
    id: Ulid,
    name: Option<String>,
    address: SocketAddr,
    writer: OwnedWriteHalf,
}

impl BrokerBot {
    pub fn has_name(&self, name: &str) -> bool {
        self.name.as_ref().map(|n| n == name).unwrap_or(false)
    }
}

pub struct BrokerClient {
    id: Ulid,
    name: Option<String>,
    is_referee: bool,
    address: SocketAddr,
    writer: OwnedWriteHalf,
}

impl BrokerClient {
    pub fn has_name(&self, name: &str) -> bool {
        self.name.as_ref().map(|n| n == name).unwrap_or(false)
    }
}

pub struct Broker {
    bots: BTreeMap<Ulid, BrokerBot>,
    clients: BTreeMap<Ulid, BrokerClient>,
}

const PING_LINE: &str = "\n";

fn is_name_valid(name: &str) -> bool {
    for c in name.chars() {
        let valid_char = (c >= 'a' && c <= 'z')
            || (c >= 'A' && c <= 'Z')
            || (c >= '0' && c <= '9')
            || (c == ' ')
            || (c == '-')
            || (c == '_')
            || (c == '.');
        if !valid_char {
            return false;
        }
    }
    if name.starts_with(" ") || name.ends_with(" ") {
        return false;
    }
    if name.to_ascii_lowercase().contains("referee") {
        return false;
    }
    true
}

impl Broker {
    pub fn new() -> Self {
        Self {
            bots: BTreeMap::new(),
            clients: BTreeMap::new(),
        }
    }

    fn bot_info(&self, id: Ulid) -> String {
        self.bots
            .get(&id)
            .map(|bot| {
                bot.name
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| bot.address.to_string())
            })
            .unwrap_or_else(|| format!("unknown bot {}", id))
    }

    fn client_info(&self, id: Ulid) -> String {
        self.clients
            .get(&id)
            .map(|c| {
                if c.is_referee {
                    if let Some(name) = &c.name {
                        format!("{}[REFEREE]", name)
                    } else {
                        "REFEREE".to_string()
                    }
                } else {
                    c.name
                        .as_ref()
                        .cloned()
                        .unwrap_or_else(|| c.address.to_string())
                }
            })
            .unwrap_or_else(|| format!("unknown client {}", id))
    }

    fn send_bot_result(&mut self, id: Ulid, sender: BrokerResultSender, result: BrokerResult) {
        if sender.send(result).is_err() {
            println!("{}: removing bot {}", Local::now(), self.bot_info(id));
            self.bots.remove(&id);
        }
    }

    fn send_client_result(&mut self, id: Ulid, sender: BrokerResultSender, result: BrokerResult) {
        if sender.send(result).is_err() {
            println!("{}: removing client {}", Local::now(), self.client_info(id));
            self.clients.remove(&id);
        }
    }

    fn remove_dead_bot(&mut self, id: Ulid) {
        println!("{}: disconected bot {}", Local::now(), self.bot_info(id));
        self.clients.remove(&id);
    }

    fn remove_dead_client(&mut self, id: Ulid) {
        println!(
            "{}: disconected client {}",
            Local::now(),
            self.client_info(id)
        );
        self.clients.remove(&id);
    }

    async fn ping_bots(&mut self) {
        let message = [0u8];
        let mut dead_bot_ids = Vec::new();
        for bot in self.bots.values_mut() {
            if bot.writer.write_all(&message).await.is_err() {
                dead_bot_ids.push(bot.id);
            }
        }
        for id in dead_bot_ids {
            self.remove_dead_bot(id);
        }

        let names = self.bots.values().fold(BTreeMap::new(), |mut names, bot| {
            if let Some(name) = bot.name.as_ref() {
                names
                    .entry(name.clone())
                    .and_modify(|n| *n += 1)
                    .or_insert(1usize);
            }
            names
        });
        for (name, count) in names.into_iter() {
            if count > 1 {
                println!(
                    "{}: warning: {} bots identified as '{}'",
                    Local::now(),
                    count,
                    name
                );
            }
        }
    }

    async fn ping_clients(&mut self) {
        let message = PING_LINE;
        let mut dead_client_ids = Vec::new();
        for bot in self.bots.values_mut() {
            if bot.writer.write_all(message.as_bytes()).await.is_err() {
                dead_client_ids.push(bot.id);
            }
        }
        for id in dead_client_ids {
            self.remove_dead_client(id);
        }

        let names = self
            .clients
            .values()
            .fold(BTreeMap::new(), |mut names, client| {
                if let Some(name) = client.name.as_ref() {
                    names
                        .entry(name.clone())
                        .and_modify(|n| *n += 1)
                        .or_insert(1usize);
                }
                names
            });
        for (name, count) in names.into_iter() {
            if count > 1 {
                println!(
                    "{}: warning: {} clients identified as '{}'",
                    Local::now(),
                    count,
                    name
                );
            }
        }
    }

    pub async fn bot_join(
        &mut self,
        id: Ulid,
        address: SocketAddr,
        writer: OwnedWriteHalf,
        sender: BrokerResultSender,
    ) {
        self.bots.insert(
            id,
            BrokerBot {
                id,
                name: None,
                address,
                writer,
            },
        );
        self.send_bot_result(id, sender, Ok(()));
    }

    pub async fn bot_name_claim(&mut self, id: Ulid, name: String, sender: BrokerResultSender) {
        self.ping_bots().await;

        let result = if !is_name_valid(&name) {
            Err(format!("invalid name '{}'", &name))
        } else {
            Ok(())
        };

        let mut dead_client_ids = Vec::new();
        if let Some(bot) = self.bots.get_mut(&id) {
            bot.name = Some(name.clone());
            let mut one_client_found = false;
            for client in
                self.clients
                    .values_mut()
                    .filter_map(|c| if c.has_name(&name) { Some(c) } else { None })
            {
                one_client_found = true;
                println!(
                    "{}: bot at address {} claims name '{}' and connects to client at address {}",
                    Local::now(),
                    bot.address,
                    &name,
                    client.address,
                );
                if client
                    .writer
                    .write_all(
                        format!(
                            "{}: connected with bot at address {}\n",
                            Local::now(),
                            bot.address
                        )
                        .as_bytes(),
                    )
                    .await
                    .is_err()
                {
                    dead_client_ids.push(client.id);
                }
            }
            if !one_client_found {
                println!(
                    "{}: bot at address {} claims name '{}' (no client)",
                    Local::now(),
                    bot.address,
                    &name,
                );
            }
        };
        for id in dead_client_ids {
            self.remove_dead_client(id);
        }

        self.send_bot_result(id, sender, result);
    }

    pub async fn join(
        &mut self,
        id: Ulid,
        address: SocketAddr,
        writer: OwnedWriteHalf,
        sender: BrokerResultSender,
    ) {
        self.clients.insert(
            id,
            BrokerClient {
                id,
                name: None,
                is_referee: false,
                address,
                writer,
            },
        );
        self.send_client_result(id, sender, Ok(()));
    }

    pub async fn name_claim(&mut self, id: Ulid, name: String, sender: BrokerResultSender) {
        self.ping_clients().await;

        let result = if !is_name_valid(&name) {
            Err(format!("invalid name '{}'", &name))
        } else {
            Ok(())
        };

        let mut messages = Vec::new();
        let mut dead_client_ids = BTreeSet::new();
        if let Some(client) = self.clients.get_mut(&id) {
            client.name = Some(name.clone());
            let mut one_bot_found = false;
            for bot in self
                .bots
                .values()
                .filter_map(|b| if b.has_name(&name) { Some(b) } else { None })
            {
                one_bot_found = true;
                messages.push(format!(
                    "{}: client at address {} claims name {} and connects to bot at address {}\n",
                    Local::now(),
                    client.address,
                    &name,
                    bot.address
                ));
            }
            if !one_bot_found {
                messages.push(format!(
                    "{}: client at address {} claims name {} (no bot)\n",
                    Local::now(),
                    client.address,
                    &name
                ))
            }

            for message in messages {
                print!("{}", &message);
                for client in self.clients.values_mut().filter_map(|c| {
                    if c.has_name(&name) {
                        Some(c)
                    } else {
                        None
                    }
                }) {
                    if client.writer.write_all(message.as_bytes()).await.is_err() {
                        dead_client_ids.insert(client.id);
                    }
                }
            }
        };
        for id in dead_client_ids {
            self.remove_dead_client(id);
        }

        self.send_client_result(id, sender, result);
    }

    pub async fn referee_claim(&mut self, id: Ulid, sender: BrokerResultSender) {
        self.ping_clients().await;

        let mut dead_client_ids = Vec::new();
        if let Some(client) = self.clients.get_mut(&id) {
            client.is_referee = true;

            let message = format!(
                "{}: client at address {} claims referee status\n",
                Local::now(),
                client.address
            );
            println!("{}", message);
            if client.writer.write_all(message.as_bytes()).await.is_err() {
                dead_client_ids.push(client.id);
            }
        }
        for id in dead_client_ids {
            self.remove_dead_client(id);
        }

        self.send_client_result(id, sender, Ok(()));
    }

    pub async fn log(&mut self, id: Ulid, time: DateTime<Local>, message: String) {
        let message = format!("{}:{}:{}\n", time, self.bot_info(id), message);
        print!("{}", &message);
        let mut dead_client_ids = Vec::new();
        for client in self.clients.values_mut() {
            if client.writer.write_all(message.as_bytes()).await.is_err() {
                dead_client_ids.push(client.id);
            }
        }
        for id in dead_client_ids {
            self.remove_dead_client(id)
        }
    }

    pub async fn referee_command(
        &mut self,
        id: Ulid,
        time: DateTime<Local>,
        command: RefereeCommand,
    ) {
        let message = format!("{}:{}:{}\n", time, self.client_info(id), command);
        print!("{}", &message);
        let encoded_command = [command.encode()];

        let mut dead_bot_ids = Vec::new();
        for bot in self.bots.values_mut() {
            if bot.writer.write_all(&encoded_command).await.is_err() {
                dead_bot_ids.push(bot.id);
            }
        }
        for id in dead_bot_ids {
            self.remove_dead_bot(id)
        }

        let mut dead_client_ids = Vec::new();
        for client in self.clients.values_mut() {
            if client.writer.write_all(message.as_bytes()).await.is_err() {
                dead_client_ids.push(client.id);
            }
        }
        for id in dead_client_ids {
            self.remove_dead_client(id)
        }
    }

    pub async fn private_command(
        &mut self,
        id: Ulid,
        time: DateTime<Local>,
        command: PrivateCommand,
    ) {
        let encoded_command = [command.encode(), '\n' as u8];
        let name = match self.clients.get(&id).and_then(|c| c.name.clone()) {
            Some(name) => name,
            None => {
                println!(
                    "{}: discarding command '{}' from unnamed client {}",
                    Local::now(),
                    command,
                    self.client_info(id)
                );
                return;
            }
        };

        let mut messages = Vec::new();
        let mut dead_bot_ids = Vec::new();
        let mut one_bot_found = false;
        for bot in self
            .bots
            .values_mut()
            .filter_map(|b| if b.has_name(&name) { Some(b) } else { None })
        {
            one_bot_found = true;
            if bot.writer.write_all(&encoded_command).await.is_err() {
                messages.push(format!(
                    "{}:{}:{} (bot unreachable)\n",
                    time, &name, command
                ));
                dead_bot_ids.push(bot.id);
            } else {
                messages.push(format!("{}:{}:{}\n", time, &name, command));
            }
        }
        if !one_bot_found {
            messages.push(format!(
                "{}:{}:{} (bot not connected)\n",
                time, &name, command
            ));
        }
        for id in dead_bot_ids {
            self.remove_dead_bot(id);
        }

        let mut dead_client_ids = BTreeSet::new();
        for message in messages {
            print!("{}", &message);
            for client in self.clients.values_mut() {
                if client.writer.write_all(message.as_bytes()).await.is_err() {
                    dead_client_ids.insert(client.id);
                }
            }
        }
        for id in dead_client_ids {
            self.remove_dead_client(id)
        }
    }

    pub async fn bot_leave(&mut self, id: Ulid) {
        self.remove_dead_bot(id)
    }

    pub async fn leave(&mut self, id: Ulid) {
        self.remove_dead_client(id)
    }
}

async fn broker_bot_listener(listener: TcpListener, sender: BrokerActionSender) {
    let broker_sender = sender;
    loop {
        match listener.accept().await {
            Ok((stream, address)) => {
                let (reader, writer) = stream.into_split();
                let id = Ulid::new();
                let (sender, receiver) = oneshot::channel();
                if broker_sender
                    .send(BrokerAction::BotJoin {
                        id,
                        address,
                        writer,
                        sender,
                    })
                    .await
                    .is_ok()
                {
                    receiver.await.ok();

                    let bot_broker_sender = broker_sender.clone();
                    let buf_reader = BufReader::new(reader);
                    let mut lines = buf_reader.lines();
                    spawn(async move {
                        loop {
                            match lines.next_line().await {
                                Ok(line) => {
                                    if let Some(mut line) = line {
                                        if let Some(l) = line.strip_suffix("\n") {
                                            line = l.to_string();
                                        }
                                        if let Some(name) = line.strip_prefix("NAME:") {
                                            let name = name.to_string();
                                            let (sender, receiver) = oneshot::channel();
                                            bot_broker_sender
                                                .send(BrokerAction::BotNameClaim {
                                                    id,
                                                    name,
                                                    sender,
                                                })
                                                .await
                                                .ok();
                                            if let Ok(result) = receiver.await {
                                                if let Err(err) = result {
                                                    println!(
                                                        "{}: bot name claim ignored: {}",
                                                        Local::now(),
                                                        err
                                                    );
                                                }
                                            } else {
                                                break;
                                            }
                                        } else {
                                            bot_broker_sender
                                                .send(BrokerAction::Log {
                                                    id,
                                                    time: Local::now(),
                                                    message: line,
                                                })
                                                .await
                                                .ok();
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                    });
                }
            }
            Err(err) => {
                println!("{}: error listening on log port: {}", Local::now(), err);
            }
        }
    }
}

async fn broker_cmd_listener(listener: TcpListener, sender: BrokerActionSender) {
    let broker_sender = sender;
    loop {
        match listener.accept().await {
            Ok((stream, address)) => {
                let (reader, writer) = stream.into_split();
                let id = Ulid::new();
                let (sender, receiver) = oneshot::channel();
                if broker_sender
                    .send(BrokerAction::Join {
                        id,
                        address,
                        writer,
                        sender,
                    })
                    .await
                    .is_ok()
                {
                    receiver.await.ok();

                    let cmd_broker_sender = broker_sender.clone();
                    let buf_reader = BufReader::new(reader);
                    let mut lines = buf_reader.lines();
                    spawn(async move {
                        loop {
                            match lines.next_line().await {
                                Ok(line) => {
                                    if let Some(mut line) = line {
                                        if let Some(l) = line.strip_suffix("\n") {
                                            line = l.to_string();
                                        }
                                        if let Some(name) = line.strip_prefix("NAME:") {
                                            let name = name.to_string();
                                            let (sender, receiver) = oneshot::channel();
                                            cmd_broker_sender
                                                .send(BrokerAction::NameClaim { id, name, sender })
                                                .await
                                                .ok();
                                            if let Ok(result) = receiver.await {
                                                if let Err(err) = result {
                                                    println!(
                                                        "{}: client name claim ignored: {}",
                                                        Local::now(),
                                                        err
                                                    );
                                                }
                                            } else {
                                                break;
                                            }
                                        } else if line == "REFEREE" {
                                            let (sender, receiver) = oneshot::channel();
                                            cmd_broker_sender
                                                .send(BrokerAction::RefereeClaim { id, sender })
                                                .await
                                                .ok();
                                            if let Ok(result) = receiver.await {
                                                if let Err(err) = result {
                                                    println!(
                                                        "{}: client referee claim ignored: {}",
                                                        Local::now(),
                                                        err
                                                    );
                                                }
                                            } else {
                                                break;
                                            }
                                        } else {
                                            if line.len() == 1 {
                                                let byte = line.chars().next().unwrap() as u8;
                                                let cmd = BotCommand::decode(byte);
                                                if let Some(cmd) = cmd {
                                                    match cmd {
                                                        BotCommand::Referee(command) => {
                                                            cmd_broker_sender
                                                                .send(
                                                                    BrokerAction::RefereeCommand {
                                                                        id,
                                                                        time: Local::now(),
                                                                        command,
                                                                    },
                                                                )
                                                                .await
                                                                .ok();
                                                        }
                                                        BotCommand::Private(command) => {
                                                            cmd_broker_sender
                                                                .send(
                                                                    BrokerAction::PrivateCommand {
                                                                        id,
                                                                        time: Local::now(),
                                                                        command,
                                                                    },
                                                                )
                                                                .await
                                                                .ok();
                                                        }
                                                    }
                                                } else {
                                                    println!(
                                                        "{}: invalid command character in '{}'",
                                                        Local::now(),
                                                        &line
                                                    );
                                                }
                                            } else {
                                                println!(
                                                    "{}: invalid command '{}'",
                                                    Local::now(),
                                                    &line
                                                );
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                    });
                }
            }
            Err(err) => {
                println!("{}: error listening on cmd port: {}", Local::now(), err);
            }
        }
    }
}

async fn broker(
    bot_port: u16,
    client_port: u16,
    args: BrokerArguments,
) -> Result<(), Box<dyn Error>> {
    let bot_addr = format!("{}:{}", &args.address, bot_port);
    let cmd_addr = format!("{}:{}", &args.address, client_port);
    let bot_listener = TcpListener::bind(&bot_addr).await?;
    let cmd_listener = TcpListener::bind(&cmd_addr).await?;
    let (broker_sender, mut broker_receiver) = mpsc::channel(32);

    spawn(broker_bot_listener(bot_listener, broker_sender.clone()));
    spawn(broker_cmd_listener(cmd_listener, broker_sender.clone()));

    let mut broker = Broker::new();

    while let Some(action) = broker_receiver.recv().await {
        match action {
            BrokerAction::BotJoin {
                id,
                address,
                writer,
                sender,
            } => {
                broker.bot_join(id, address, writer, sender).await;
            }
            BrokerAction::BotNameClaim { id, name, sender } => {
                broker.bot_name_claim(id, name, sender).await;
            }
            BrokerAction::Join {
                id,
                address,
                writer,
                sender,
            } => {
                broker.join(id, address, writer, sender).await;
            }
            BrokerAction::NameClaim { id, name, sender } => {
                broker.name_claim(id, name, sender).await;
            }
            BrokerAction::RefereeClaim { id, sender } => {
                broker.referee_claim(id, sender).await;
            }
            BrokerAction::Log { id, time, message } => {
                broker.log(id, time, message).await;
            }
            BrokerAction::RefereeCommand { id, time, command } => {
                broker.referee_command(id, time, command).await;
            }
            BrokerAction::PrivateCommand { id, time, command } => {
                broker.private_command(id, time, command).await;
            }
            BrokerAction::BotLeave { id } => {
                broker.bot_leave(id).await;
            }
            BrokerAction::Leave { id } => {
                broker.leave(id).await;
            }
        }
    }

    Ok(())
}

async fn cmd_client(
    client_port: u16,
    address: String,
    name: String,
    is_referee: bool,
    is_private: bool,
) -> Result<(), Box<dyn Error>> {
    let addr = format!("{}:{}", &address, client_port);
    let stream = TcpStream::connect(&addr).await?;
    let (log_stream, mut cmd_stream) = stream.into_split();

    spawn(async move {
        let log_reader = BufReader::new(log_stream);
        let mut lines = log_reader.lines();
        loop {
            match lines.next_line().await {
                Ok(line) => {
                    if let Some(line) = line {
                        if line != PING_LINE {
                            println!("{}", line);
                        }
                    } else {
                        println!("{}: logs terminated", Local::now());
                        break;
                    }
                }
                Err(err) => {
                    println!("{}: error reading logs: {}", Local::now(), err);
                    break;
                }
            }
        }
    });

    if is_private {
        let line = format!("NAME:{}\n", name);
        cmd_stream.write_all(line.as_bytes()).await?;
    }

    if is_referee {
        cmd_stream.write_all("REFEREE\n".as_bytes()).await?;
    }

    let stdin_reader = BufReader::new(tokio::io::stdin());
    let mut stdin_lines = stdin_reader.lines();
    loop {
        match stdin_lines.next_line().await {
            Ok(line) => {
                if let Some(mut line) = line {
                    line.push('\n');
                    cmd_stream.write_all(line.as_bytes()).await?;
                } else {
                    println!("stdin terminated, exiting");
                    break;
                }
            }
            Err(err) => {
                println!("error reading from stdin, exiting: {}", err);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let global_args = Arguments::parse();

    match global_args.action {
        SubCommand::Broker(args) => {
            broker(global_args.bot_port, global_args.client_port, args).await
        }
        SubCommand::Referee(args) => {
            cmd_client(
                global_args.client_port,
                args.address,
                "REFEREE".to_string(),
                true,
                false,
            )
            .await
        }
        SubCommand::Cmd(args) => {
            cmd_client(
                global_args.client_port,
                args.address,
                args.name,
                args.referee,
                true,
            )
            .await
        }
    }?;

    Ok(())
}

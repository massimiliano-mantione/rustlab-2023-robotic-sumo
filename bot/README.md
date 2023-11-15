This is the environment to build code that will run on the robot.

Some of the programs require a wifi SSID and secret to build, but I did not put any in the git repo.
Doing so would have been error prone: one risks to use their own private wifi credentials, and then commit and push them by mistake.

To build all examples you must create two files: `src/WIFI_SSID.txt` and `src/WIFI_SECRET.txt`.
Then the full build will succeed.
Otherwise just build single programs that do not connect to the wifi, like `cargo build --bin rustlab-sumo`.

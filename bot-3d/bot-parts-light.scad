$fn = 64;

M2_GRIP_HOLE = 2.3;
M2_FREE_HOLE = 2.45;

M4_GRIP_HOLE = 4.3;
M4_FREE_HOLE = 4.45;

DRIVER_BOARD_W = 35;
DRIVER_BOARD_L = 63.5;
DRIVER_BOARD_THICK = 1.5;
DRIVER_BOARD_HOLE_D = 3;
DRIVER_BOARD_TOP_HOLE_OFF = 3.5;
DRIVER_BOARD_CENTER_HOLE_W_OFF = 1;
DRIVER_BOARD_CENTER_HOLE_L_OFF = 5;

BATTERY_W = 64;
BATTERY_L = 91;
BATTERY_THICK = 13;
BATTERY_CORNER_R = 6.5;
BATTERY_BUTTON_OFFSET = 15;
BATTERY_BUTTON_D = 4.5;
BATTERY_LEDS_W = 20;
BATTERY_LEDS_L = 10;
BATTERY_LEDS_C_OFF = 16;

MOTOR_W = 20;
MOTOR_H = 23;
MOTOR_BOX_L = 37;
MOTOR_BOX_CORNER_R = 2;
MOTOR_NECK_L = 11;
MOTOR_BODY_L = MOTOR_BOX_L + MOTOR_NECK_L;
MOTOR_L = 65;
MOTOR_HOOK_W = 3;
MOTOR_HOOK_H = 10;
MOTOR_HOOK_L = 6;
MOTOR_AXLE_POS = 11.5;
MOTOR_AXLE_R = 5;
MOTOR_AXLE_L = MOTOR_W + (10 * 2);
MOTOR_PIN_POS = MOTOR_AXLE_POS + 11;
MOTOR_PIN_R = 3;
MOTOR_PIN_L = MOTOR_W + (10 * 2);
MOTOR_SIDE_SNAP_W = MOTOR_W + 10;
MOTOR_SIDE_SNAP_L = 4;
MOTOR_SIDE_SNAP_H = 6;
MOTOR_SIDE_SNAP_POS = 42.5;
MOTOR_TOP_SNAP_W = 6;
MOTOR_TOP_SNAP_L = 6;
MOTOR_TOP_SNAP_H = MOTOR_H + 10;
MOTOR_TOP_SNAP_POS = 53;

WHEEL_D = 62.5;
WHEEL_W = 38;

SENSOR_BOARD_W = 14.5;
SENSOR_BOARD_L = 31.5;
SENSOR_BOARD_THICK = 1;
SENSOR_BOARD_HOLE_CENTER_OFF = 7.4;
SENSOR_BOARD_HOLE_D = DRIVER_BOARD_HOLE_D;
SENSOR_BOARD_TOP_H = 20;
SENSOR_BOARD_BOTTOM_H = 15;
SENSOR_BOARD_BOTTOM_W = 12.5;
SENSOR_BOARD_BOTTOM_L = 10;
SENSOR_BOARD_CONNECTOR_L = 15;
SENSOR_BOARD_CONNECTOR_W = 10;

SENSOR_BOARD_PIN_H = 5;
SENSOR_BOARD_BACK_PIN_W = SENSOR_BOARD_CONNECTOR_W;
SENSOR_BOARD_BACK_PIN_L = 3;
SENSOR_BOARD_BACK_PIN_OFF = 1.5;
SENSOR_BOARD_MID_PIN_W = 12;
SENSOR_BOARD_MID_PIN_L = 8;
SENSOR_BOARD_MID_PIN_OFF = 15.5;

DISTANCE_BOARD_W = 17;
DISTANCE_BOARD_L = 41.5;
DISTANCE_BOARD_THICK = 1;
DISTANCE_BOARD_HOLE_CENTER_OFF = DISTANCE_BOARD_L - 15.5;
DISTANCE_BOARD_HOLE_D = DRIVER_BOARD_HOLE_D;
DISTANCE_BOARD_TOP_H = 20;
DISTANCE_BOARD_CONNECTOR_L = 15;
DISTANCE_BOARD_CONNECTOR_W = 10;
DISTANCE_BOARD_SENSOR_W = 15;
DISTANCE_BOARD_SENSOR_L = 30;

DISTANCE_BOARD_OFFSET_W = 18;
DISTANCE_BOARD_OFFSET_L = 28.5;
DISTANCE_BOARD_ANGLE = 15;

COLOR_BOARD_OFFSET_L = 19.5;

MOTOR_W_OFF = 10;
MOTOR_L_OFF = MOTOR_BODY_L;

BACK_CONNECTOR_R = 5;
FRONT_CONNECTOR_R = 7;
FRONT_CONNECTOR_OFF = (SENSOR_BOARD_W/2) + FRONT_CONNECTOR_R;

WALL_THICK = 2;
FLOOR_THICK = 2;


BATTERY_BOX_W = BATTERY_W + (WALL_THICK * 2);
BATTERY_BOX_L = BATTERY_L + (WALL_THICK * 2);

BACK_BOX_W = BATTERY_W + (WALL_THICK * 2) + (MOTOR_W_OFF * 2);
BACK_BOX_L = MOTOR_L_OFF + MOTOR_NECK_L + (WALL_THICK * 2);
BACK_BOX_L_OFF = -MOTOR_NECK_L;

module motor() {
    translate([0, 0, MOTOR_H / 2])
    rotate([0, 90, 180])
    union() {
        cube([MOTOR_HOOK_H, MOTOR_HOOK_L * 2, MOTOR_HOOK_W], center=true);
        translate([0, MOTOR_AXLE_POS, 0]) cylinder(r=MOTOR_AXLE_R, h=MOTOR_AXLE_L, center=true);
        translate([0, MOTOR_PIN_POS, 0]) cylinder(r=MOTOR_PIN_R, h=MOTOR_PIN_L, center=true);
        translate([0, MOTOR_SIDE_SNAP_POS, 0]) cube([MOTOR_SIDE_SNAP_H, MOTOR_SIDE_SNAP_L, MOTOR_SIDE_SNAP_W], center=true);
        translate([0, MOTOR_TOP_SNAP_POS, 0]) cube([MOTOR_TOP_SNAP_H, MOTOR_TOP_SNAP_L, MOTOR_TOP_SNAP_W], center=true);
        intersection() {
            union() {
                translate([+ (MOTOR_H / 2) - MOTOR_BOX_CORNER_R, MOTOR_BOX_CORNER_R, 0]) cylinder(r=MOTOR_BOX_CORNER_R, h=MOTOR_W, center=true);
                translate([- (MOTOR_H / 2) + MOTOR_BOX_CORNER_R, MOTOR_BOX_CORNER_R, 0]) cylinder(r=MOTOR_BOX_CORNER_R, h=MOTOR_W, center=true);
                rotate([-90, 0, 0]) union() {
                    translate([0, 0, MOTOR_BOX_CORNER_R + MOTOR_BODY_L / 2]) cylinder(r=(MOTOR_H + 2.5) / 2, h=MOTOR_BODY_L - MOTOR_BOX_CORNER_R, center=true);
                    translate([0, 0, MOTOR_BOX_CORNER_R + MOTOR_L / 2]) cylinder(r=(MOTOR_H + 0.5) / 2, h=MOTOR_L - MOTOR_BOX_CORNER_R, center=true);
                    translate([0, 0, MOTOR_BOX_CORNER_R + (MOTOR_BOX_L - MOTOR_BOX_CORNER_R) / 2]) cube([MOTOR_H, MOTOR_W, MOTOR_BOX_L - MOTOR_BOX_CORNER_R], center=true);
                    translate([0, 0, MOTOR_BOX_L / 2]) cube([MOTOR_H - (MOTOR_BOX_CORNER_R * 2), MOTOR_W, MOTOR_BOX_L], center=true);
                }
            }
            rotate([-90, 0, 0]) translate([0, 0, MOTOR_L / 2]) cube([MOTOR_H, MOTOR_W, MOTOR_L], center=true);
        }
    }
}

module battery() {
    translate([0, BATTERY_L / 2, BATTERY_THICK / 2])
    union() {
        cube([BATTERY_W - (BATTERY_CORNER_R * 2), BATTERY_L, BATTERY_THICK], center=true);
        cube([BATTERY_W, BATTERY_L - (BATTERY_CORNER_R * 2), BATTERY_THICK], center=true);
        translate([0, -(BATTERY_L + 20)/2, 0]) cube([BATTERY_W - (BATTERY_CORNER_R * 2), 22, BATTERY_THICK], center=true);
        translate([+ (BATTERY_W / 2) - BATTERY_CORNER_R, + (BATTERY_L / 2) - BATTERY_CORNER_R, 0]) cylinder(r=BATTERY_CORNER_R, h=BATTERY_THICK, center=true);
        translate([+ (BATTERY_W / 2) - BATTERY_CORNER_R, - (BATTERY_L / 2) + BATTERY_CORNER_R, 0]) cylinder(r=BATTERY_CORNER_R, h=BATTERY_THICK, center=true);
        translate([- (BATTERY_W / 2) + BATTERY_CORNER_R, + (BATTERY_L / 2) - BATTERY_CORNER_R, 0]) cylinder(r=BATTERY_CORNER_R, h=BATTERY_THICK, center=true);
        translate([- (BATTERY_W / 2) + BATTERY_CORNER_R, - (BATTERY_L / 2) + BATTERY_CORNER_R, 0]) cylinder(r=BATTERY_CORNER_R, h=BATTERY_THICK, center=true);
    }
}

module distance_sensor() {
    CONNECTOR_H = DISTANCE_BOARD_TOP_H - DISTANCE_BOARD_THICK;
    SENSOR_H = CONNECTOR_H;
    // `difference` for pin instead of hole, otherwise `union`
    union() {
        translate([0, DISTANCE_BOARD_L / 2, 0])
        union() {
            translate([0, 0, DISTANCE_BOARD_TOP_H/2])
                cube([DISTANCE_BOARD_W, DISTANCE_BOARD_L, DISTANCE_BOARD_TOP_H], center=true);
            translate([0, -(DISTANCE_BOARD_CONNECTOR_L/2), (CONNECTOR_H/2) + DISTANCE_BOARD_THICK])
                cube([DISTANCE_BOARD_CONNECTOR_W + 3, DISTANCE_BOARD_L + (DISTANCE_BOARD_CONNECTOR_L * 1), CONNECTOR_H], center=true);
            translate([0, (DISTANCE_BOARD_SENSOR_L/2), (SENSOR_H/2) + DISTANCE_BOARD_THICK])
                cube([DISTANCE_BOARD_SENSOR_W, DISTANCE_BOARD_SENSOR_L + (DISTANCE_BOARD_SENSOR_L * 1), SENSOR_H], center=true);
        };
        // pin cylinder
        // translate([0, SENSOR_BOARD_HOLE_CENTER_OFF, SENSOR_BOARD_THICK/2]) cylinder(r=SENSOR_BOARD_HOLE_D/2, h=SENSOR_BOARD_THICK, center=true);
        translate([0, DISTANCE_BOARD_HOLE_CENTER_OFF, -10]) cylinder(r=DISTANCE_BOARD_HOLE_D/2, h=20, center=true);
    }
}

module color_sensor() {
    translate([0, SENSOR_BOARD_BACK_PIN_OFF, 0.01 + -SENSOR_BOARD_PIN_H/2]) cube([SENSOR_BOARD_BACK_PIN_W, SENSOR_BOARD_BACK_PIN_L, SENSOR_BOARD_PIN_H], center=true);
    translate([0, SENSOR_BOARD_MID_PIN_OFF, 0.01 + -SENSOR_BOARD_PIN_H/2]) cube([SENSOR_BOARD_MID_PIN_W, SENSOR_BOARD_MID_PIN_L, SENSOR_BOARD_PIN_H], center=true);
    // `difference` for pin instead of hole, otherwise `union`
    union() {
        translate([0, SENSOR_BOARD_L / 2, 0])
        union() {
            translate([0, 0, SENSOR_BOARD_TOP_H/2]) cube([SENSOR_BOARD_W, SENSOR_BOARD_L, SENSOR_BOARD_TOP_H], center=true);
            translate([0, -(SENSOR_BOARD_CONNECTOR_L/2), (SENSOR_BOARD_TOP_H + SENSOR_BOARD_THICK)/2]) cube([SENSOR_BOARD_CONNECTOR_W, SENSOR_BOARD_L + (SENSOR_BOARD_CONNECTOR_L * 1), SENSOR_BOARD_TOP_H - SENSOR_BOARD_THICK], center=true);
            translate([0, (SENSOR_BOARD_L-SENSOR_BOARD_BOTTOM_L)/2, -SENSOR_BOARD_BOTTOM_H/2]) cube([SENSOR_BOARD_BOTTOM_W, SENSOR_BOARD_BOTTOM_L, SENSOR_BOARD_BOTTOM_H], center=true);
        };
        // pin cylinder
        // translate([0, SENSOR_BOARD_HOLE_CENTER_OFF, SENSOR_BOARD_THICK/2]) cylinder(r=SENSOR_BOARD_HOLE_D/2, h=SENSOR_BOARD_THICK, center=true);
        translate([0, SENSOR_BOARD_HOLE_CENTER_OFF, -10]) cylinder(r=SENSOR_BOARD_HOLE_D/2, h=20, center=true);
    }
}


module driver_board_holes() {
    H=30;
    translate([0, DRIVER_BOARD_POS, 0]) {
        translate([DRIVER_BOARD_CENTER_HOLE_W_OFF, DRIVER_BOARD_L-DRIVER_BOARD_CENTER_HOLE_L_OFF, H]) cylinder(r=DRIVER_BOARD_HOLE_D/2, h=H*3, center=true);
        translate([+(DRIVER_BOARD_W/2) -DRIVER_BOARD_TOP_HOLE_OFF, DRIVER_BOARD_TOP_HOLE_OFF, H]) cylinder(r=DRIVER_BOARD_HOLE_D/2, h=H*3, center=true);
        translate([-(DRIVER_BOARD_W/2) +DRIVER_BOARD_TOP_HOLE_OFF, DRIVER_BOARD_TOP_HOLE_OFF, H]) cylinder(r=DRIVER_BOARD_HOLE_D/2, h=H*3, center=true);
    }
}

module driver_board() {
    difference() {
        translate([0, DRIVER_BOARD_POS, 0]) translate([0, DRIVER_BOARD_L/2, DRIVER_BOARD_THICK/2]) cube([DRIVER_BOARD_W, DRIVER_BOARD_L, DRIVER_BOARD_THICK], center=true);
        driver_board_holes();
    }
}


module connector(thick, radius) {
    difference() {
        union() {
            translate([0, radius/2, thick/2]) cube([(radius*2), radius, thick], center=true);
            translate([0, radius, thick/2]) cylinder(r=radius, h=thick, center=true);
        }
        translate([0, radius, thick/2]) cylinder(r=M4_FREE_HOLE/2, h=thick*2, center=true);
    }
}

module front_connectors(thick) {
    // WALL = CONNECTOR_R - (M4_FREE_HOLE/2);
    WALL = 2;
    RADIUS = FRONT_CONNECTOR_R;
    FLAT_L_OUT = 3;
    FLAT_L_IN = (RADIUS*2) - FLAT_L_OUT;
    FLAT_THICK = 3;
    FLAT_L_EXTRA = (BATTERY_BOX_W / 2) - (FRONT_CONNECTOR_OFF + RADIUS);
    
    translate([0, BATTERY_L, 0]) {
        translate([0, (RADIUS*2) - (WALL/2), thick/2]) cube([(FRONT_CONNECTOR_OFF + 0) * 2, WALL, thick], center=true);
        translate([+FRONT_CONNECTOR_OFF - RADIUS + (WALL/2), RADIUS, thick/2]) cube([WALL, (RADIUS*2), thick], center=true);
        translate([-FRONT_CONNECTOR_OFF + RADIUS - (WALL/2), RADIUS, thick/2]) cube([WALL, (RADIUS*2), thick], center=true);
        translate([+FRONT_CONNECTOR_OFF, 0, 0]) connector(thick, RADIUS);
        translate([-FRONT_CONNECTOR_OFF, 0, 0]) connector(thick, RADIUS);
    }
}

module front_connectors_flats(thick) {
    // WALL = CONNECTOR_R - (M4_FREE_HOLE/2);
    WALL = 2;
    RADIUS = FRONT_CONNECTOR_R;
    FLAT_L_OUT = 3;
    FLAT_L_IN = (RADIUS*2) - FLAT_L_OUT;
    FLAT_THICK = 3;
    FLAT_L_EXTRA = (BATTERY_BOX_W / 2) - (FRONT_CONNECTOR_OFF + RADIUS);
    
    translate([0, BATTERY_L, 0]) {
        translate([+FRONT_CONNECTOR_OFF, (RADIUS*2) - (FLAT_L_OUT/2), FLAT_THICK/2]) cube([(RADIUS*2), FLAT_L_OUT, FLAT_THICK], center=true);
        translate([-FRONT_CONNECTOR_OFF, (RADIUS*2) - (FLAT_L_OUT/2), FLAT_THICK/2]) cube([(RADIUS*2), FLAT_L_OUT, FLAT_THICK], center=true);
        translate([+FRONT_CONNECTOR_OFF + RADIUS - (WALL/2), RADIUS, FLAT_THICK/2]) cube([WALL, (RADIUS*2), FLAT_THICK], center=true);
        translate([-FRONT_CONNECTOR_OFF - RADIUS + (WALL/2), RADIUS, FLAT_THICK/2]) cube([WALL, (RADIUS*2), FLAT_THICK], center=true);
        translate([+ (FRONT_CONNECTOR_OFF + RADIUS + (FLAT_L_EXTRA/2)), RADIUS, FLAT_THICK/2]) cube([FLAT_L_EXTRA, (RADIUS*2), FLAT_THICK], center=true);
        translate([- (FRONT_CONNECTOR_OFF + RADIUS + (FLAT_L_EXTRA/2)), RADIUS, FLAT_THICK/2]) cube([FLAT_L_EXTRA, (RADIUS*2), FLAT_THICK], center=true);
        translate([+BATTERY_BOX_W/2, RADIUS, FLAT_THICK/2]) cylinder(d=RADIUS*2, h=FLAT_THICK, center=true);
        translate([-BATTERY_BOX_W/2, RADIUS, FLAT_THICK/2]) cylinder(d=RADIUS*2, h=FLAT_THICK, center=true);
    }
}

module back_connectors(thick) {
    RADIUS = BACK_CONNECTOR_R;
    translate([+(BATTERY_W/2), MOTOR_L_OFF + RADIUS, 0]) rotate([0, 0, -90]) connector(thick, RADIUS);
    translate([-(BATTERY_W/2), MOTOR_L_OFF + RADIUS, 0]) rotate([0, 0, +90]) connector(thick, RADIUS);
}


module front_box(thick) {
    translate([0, (BATTERY_BOX_L/2) - WALL_THICK, thick/2]) cube([BATTERY_BOX_W, BATTERY_BOX_L, thick], center=true);
}

module back_box(thick) {
    translate([0, (BACK_BOX_L/2) - WALL_THICK + BACK_BOX_L_OFF, thick/2]) cube([BACK_BOX_W, BACK_BOX_L, thick], center=true);
}

module battery_box() {
    H = BATTERY_THICK + FLOOR_THICK;

    B_HOLE_W = 20;
    B_HOLE_H = 20;
    B_HOLE_L = BATTERY_BUTTON_OFFSET + BATTERY_BUTTON_D + WALL_THICK - BACK_BOX_L_OFF + 0.1;

    difference() {
        union() {
            front_connectors(H);
            back_connectors(H);
            front_box(H);
            back_box(H);
        }
        union() {
            translate([0, 0, FLOOR_THICK + 0.01]) battery();
            translate([(+B_HOLE_W + BATTERY_W)/2 - 0.01, -0.01 + (B_HOLE_L/2) + BACK_BOX_L_OFF - WALL_THICK, B_HOLE_H/2]) cube([B_HOLE_W, B_HOLE_L, B_HOLE_H*2], center=true);
            translate([(-B_HOLE_W - BATTERY_W)/2 + 0.01, -0.01 + (B_HOLE_L/2) + BACK_BOX_L_OFF - WALL_THICK, B_HOLE_H/2]) cube([B_HOLE_W, B_HOLE_L, B_HOLE_H*2], center=true);
            translate([0, BATTERY_LEDS_C_OFF, 0]) cube([BATTERY_LEDS_W*2, BATTERY_LEDS_L, 20], center=true);

            translate([0, 56, 0]) cube([55, 60, 40], center=true);

            translate([0, -10, 0]) cube([BACK_BOX_W,20,2*H], center=true);
            translate([0, 29, B_HOLE_H/2]) cube([75,40,H], center=true);
        }
    }
}

MOTOR_BOX_HOLE_LEVEL = 2;
MOTOR_BOX_HOLE_H = 20;
MOTOR_BOX_FRONT_HOLE_W = BATTERY_W;
MOTOR_BOX_FRONT_HOLE_L = 35;
MOTOR_BOX_BACK_HOLE_W = 40;
MOTOR_BOX_BACK_HOLE_L = 56;

module motor_box_bottom() {
    H = (MOTOR_H / 2) + FLOOR_THICK;

    BOTTOM_HOLE_W = MOTOR_BOX_BACK_HOLE_W - 4;
    BOTTOM_HOLE_L = MOTOR_BOX_BACK_HOLE_L - 2;

    difference() {
        union() {
            front_connectors(H);
            front_connectors_flats(H);
            back_connectors(H);
            front_box(H);
            back_box(H);
        }
        translate([0, 0, FLOOR_THICK]) union() {
            translate([(+ (BATTERY_W - MOTOR_W) / 2) + MOTOR_W_OFF, MOTOR_L_OFF, 0]) motor();
            translate([(- (BATTERY_W - MOTOR_W) / 2) - MOTOR_W_OFF, MOTOR_L_OFF, 0]) motor();
            translate([0, BATTERY_L - COLOR_BOARD_OFFSET_L, 0]) color_sensor();
            translate([+DISTANCE_BOARD_OFFSET_W, BATTERY_L - DISTANCE_BOARD_OFFSET_L, 0]) rotate([0, 0, -DISTANCE_BOARD_ANGLE]) distance_sensor();
            translate([-DISTANCE_BOARD_OFFSET_W, BATTERY_L - DISTANCE_BOARD_OFFSET_L, 0]) rotate([0, 0, +DISTANCE_BOARD_ANGLE]) distance_sensor();
            translate([0, BATTERY_L - (MOTOR_BOX_FRONT_HOLE_L/2), MOTOR_BOX_HOLE_LEVEL + (MOTOR_BOX_HOLE_H / 2)]) cube([MOTOR_BOX_FRONT_HOLE_W, MOTOR_BOX_FRONT_HOLE_L, 
MOTOR_BOX_HOLE_H], center=true);
            translate([0, -0.01 - WALL_THICK + BACK_BOX_L_OFF + (MOTOR_BOX_BACK_HOLE_L/2), MOTOR_BOX_HOLE_LEVEL + (MOTOR_BOX_HOLE_H / 2)]) cube([MOTOR_BOX_BACK_HOLE_W, MOTOR_BOX_BACK_HOLE_L, MOTOR_BOX_HOLE_H], center=true);
            translate([0, -0.01 - WALL_THICK + BACK_BOX_L_OFF + (BOTTOM_HOLE_L/2), -H]) cube([BOTTOM_HOLE_W, BOTTOM_HOLE_L, H*3], center=true);
            
            translate([0, -10, 0]) cube([BACK_BOX_W,40,2*H], center=true);
        }
    }
}

DRIVER_BOARD_POS = -11;

module motor_box_top() {
    H = (MOTOR_H / 2) + FLOOR_THICK;
    BACK_HOLE_L = MOTOR_BOX_BACK_HOLE_L + 20;
    SERVICE_HOLE_W = MOTOR_BOX_FRONT_HOLE_W - 3;
    SERVICE_HOLE_L = MOTOR_BOX_FRONT_HOLE_L - 3;

    difference() {
        union() {
            front_connectors(H);
            front_connectors_flats(H);
            back_connectors(H);
            front_box(H);
            back_box(H);
        }
        translate([0, 0, FLOOR_THICK]) union() {
            translate([(+ (BATTERY_W - MOTOR_W) / 2) + MOTOR_W_OFF, MOTOR_L_OFF, 0]) motor();
            translate([(- (BATTERY_W - MOTOR_W) / 2) - MOTOR_W_OFF, MOTOR_L_OFF, 0]) motor();

            translate([+DISTANCE_BOARD_OFFSET_W, BATTERY_L - DISTANCE_BOARD_OFFSET_L, FLOOR_THICK]) rotate([0, 0, -DISTANCE_BOARD_ANGLE]) distance_sensor();
            translate([-DISTANCE_BOARD_OFFSET_W, BATTERY_L - DISTANCE_BOARD_OFFSET_L, FLOOR_THICK]) rotate([0, 0, +DISTANCE_BOARD_ANGLE]) distance_sensor();

            translate([0, BATTERY_L - (MOTOR_BOX_FRONT_HOLE_L/2), MOTOR_BOX_HOLE_LEVEL + (MOTOR_BOX_HOLE_H / 2)]) cube([MOTOR_BOX_FRONT_HOLE_W, MOTOR_BOX_FRONT_HOLE_L, MOTOR_BOX_HOLE_H], center=true);
            translate([0, -0.01 - WALL_THICK + BACK_BOX_L_OFF + (BACK_HOLE_L/2), MOTOR_BOX_HOLE_LEVEL + (MOTOR_BOX_HOLE_H / 2)]) cube([MOTOR_BOX_BACK_HOLE_W, BACK_HOLE_L, MOTOR_BOX_HOLE_H], center=true);
            translate([0, -3 + BATTERY_L - (SERVICE_HOLE_L/2), 0]) cube([SERVICE_HOLE_W, SERVICE_HOLE_L, MOTOR_BOX_HOLE_H], center=true);
            driver_board_holes();

            translate([0, 20, 0]) cube([28, 46, 40], center=true);

            translate([-BACK_BOX_W/2, -10, 0]) cube([BACK_BOX_W/2,40,2*H], center=true);       
translate([+BACK_BOX_W/2, -10, 0]) cube([BACK_BOX_W/2,40,2*H], center=true);        
        }
    }
}

FOOT_H = 3;
FOOT_R = 10;

module foot() {
    difference() {
        translate([0, 0, FOOT_H - FOOT_R]) sphere(r=FOOT_R);
        translate([0, 0, FLOOR_THICK -10]) cube([20, 20, 20], center=true);
    }
}

module bottom_frame() {
    FOOT_L = 10;
    FOOT_W_OFF = 15;
    
    HOLE_W = MOTOR_BOX_FRONT_HOLE_W - 3;
    HOLE_L = BATTERY_L - BACK_BOX_L_OFF - FOOT_L;
    B_HOLE_L = MOTOR_L_OFF - BACK_BOX_L_OFF;

    CONE_R1 = M4_FREE_HOLE/2;
    CONE_R2 = CONE_R1 + 3;

    translate([+FOOT_W_OFF, HOLE_L - (FOOT_L / 2), 0]) foot();
    translate([-FOOT_W_OFF, HOLE_L - (FOOT_L / 2), 0]) foot();
    difference() {
        union() {
            front_connectors(FLOOR_THICK);
            back_connectors(FLOOR_THICK);
            front_box(FLOOR_THICK);
            back_box(FLOOR_THICK);
        }
        union() {
            translate([0, BACK_BOX_L_OFF - WALL_THICK + (HOLE_L/2) - 0.1, 0]) cube([HOLE_W, HOLE_L, 20], center=true);
            translate([0, (B_HOLE_L/2) + BACK_BOX_L_OFF - WALL_THICK - 0.1, 0]) cube([100, B_HOLE_L, 20], center=true);
            translate([+FRONT_CONNECTOR_OFF, BATTERY_L + FRONT_CONNECTOR_R, - 0.01]) cylinder(r1=CONE_R1, r2=CONE_R2, h=FLOOR_THICK + 0.02);
            translate([-FRONT_CONNECTOR_OFF, BATTERY_L + FRONT_CONNECTOR_R, - 0.01]) cylinder(r1=CONE_R1, r2=CONE_R2, h=FLOOR_THICK + 0.02);
            translate([+(BATTERY_W/2) + BACK_CONNECTOR_R, MOTOR_L_OFF + BACK_CONNECTOR_R, -0.01]) cylinder(r1=CONE_R1, r2=CONE_R2, h=FLOOR_THICK + 0.02);
            translate([-(BATTERY_W/2) - BACK_CONNECTOR_R, MOTOR_L_OFF + BACK_CONNECTOR_R, -0.01]) cylinder(r1=CONE_R1, r2=CONE_R2, h=FLOOR_THICK + 0.02);
        }
    }
}

module top_pillars(h, w) {
    translate([+(BACK_BOX_W-w)/2, w/2 + BACK_BOX_L_OFF - WALL_THICK, h/2]) cube([w, w, h], center=true);
    translate([-(BACK_BOX_W-w)/2, w/2 + BACK_BOX_L_OFF - WALL_THICK, h/2]) cube([w, w, h], center=true);

    translate([+(BACK_BOX_W-w)/2, -w/2 + MOTOR_L_OFF, h/2]) cube([w, w, h], center=true);
    translate([-(BACK_BOX_W-w)/2, -w/2 + MOTOR_L_OFF, h/2]) cube([w, w, h], center=true);

    translate([+(BATTERY_BOX_W-w)/2, -w/2 + BATTERY_BOX_L - WALL_THICK, h/2]) cube([w, w, h], center=true);
    translate([-(BATTERY_BOX_W-w)/2, -w/2 + BATTERY_BOX_L - WALL_THICK, h/2]) cube([w, w, h], center=true);
}

module top_frame() {
    FRAME_H = 3;
    PILLAR_H1 = 25;
    PILLAR_H2 = PILLAR_H1 + FLOOR_THICK;
    PILLAR_W1 = 5;
    PILLAR_W2 = WALL_THICK;

    F_HOLE_W_UP = BATTERY_W;
    F_HOLE_L_UP = BATTERY_L - BACK_BOX_L_OFF;
    B_HOLE_W_UP = BACK_BOX_W - WALL_THICK;
    B_HOLE_L_UP = BACK_BOX_L - (BACK_CONNECTOR_R * 2) - BACK_BOX_L_OFF - (WALL_THICK* 2) - 3;

    F_HOLE_W = F_HOLE_W_UP - 6;
    F_HOLE_L = F_HOLE_L_UP - 3;
    B_HOLE_W = B_HOLE_W_UP - 9;
    B_HOLE_L = B_HOLE_L_UP;

    difference() {
        union() {
            front_connectors(FRAME_H);
            back_connectors(FRAME_H);
            front_box(FRAME_H);
            back_box(FRAME_H);
            top_pillars(PILLAR_H1, PILLAR_W1);
            top_pillars(PILLAR_H2, PILLAR_W2);
        }
        union() {
            translate([0, B_HOLE_L/2 + BACK_BOX_L_OFF - 0.1, 0]) cube([B_HOLE_W, B_HOLE_L, 100], center=true);
            translate([0, F_HOLE_L/2 + BACK_BOX_L_OFF - WALL_THICK - 0.1, 0]) cube([F_HOLE_W, F_HOLE_L, 100], center=true);

            translate([0, B_HOLE_L_UP/2 + BACK_BOX_L_OFF - 0.1, 50 + FLOOR_THICK]) cube([B_HOLE_W_UP, B_HOLE_L_UP, 100], center=true);
            translate([0, F_HOLE_L_UP/2 + BACK_BOX_L_OFF - WALL_THICK - 0.1, 50 + FLOOR_THICK]) cube([F_HOLE_W_UP, F_HOLE_L_UP, 100], center=true);
        }
    }
}

module assembly() {
    translate([0, 0, -5]) rotate([0, 180, 0]) bottom_frame();
    translate([0, 0, 10]) battery_box();
    translate([0, 0, 40]) motor_box_bottom();
    translate([0, 0, 80]) rotate([0, 180, 0]) motor_box_top();
    translate([0, 0, 95]) top_frame();
}

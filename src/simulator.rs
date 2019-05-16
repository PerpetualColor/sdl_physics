pub mod util;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use util::PhysVector;

pub enum SimulateFunction {
    Gravity,
    GravityResistive,
    Butterfly,
    WindowsXP,
    Logistic,
    InverseSquare,
    Harmonic,
    NoForce
}

pub struct Simulator {
    pub particle_list: Rc<RefCell<Vec<Particle>>>,
    steps: usize,
    timestep: f32,
    function: SimulateFunction,
    pub bounce_off_walls: bool,
}

/* a simulated particle */
pub struct Particle {
    pos: PhysVector,
    vel: PhysVector,
}

impl Simulator {
    pub fn new(timestep: f32) -> Simulator {
        Simulator {
            particle_list: Rc::new(RefCell::new(Vec::new())),
            steps: 0,
            timestep: timestep,
            function: SimulateFunction::Gravity,
            bounce_off_walls: false
        }
    }

    pub fn clear(&mut self) {
        self.particle_list = Rc::new(RefCell::new(Vec::new()));
    }

    pub fn set_function(&mut self, func: SimulateFunction) {
        self.function = func;
    }

    pub fn step(simulator: Rc<RefCell<Simulator>>, count: usize) {
        let avg_velocity = false;
        {
            let sim = simulator.borrow();
            let particle_list = &mut sim.particle_list.borrow_mut();
            for p in particle_list.iter_mut() {
                let prev_vel = p.vel.clone();
                for _i in 0..count {
                    p.vel = &(&sim.acceleration_for(&p) * sim.timestep.into()) + &p.vel;
                    if avg_velocity {
                        p.pos = &(&(&p.vel + &prev_vel) * (0.5 * sim.timestep)) + &p.pos;
                    } else {
                        p.pos = &(&p.vel * sim.timestep) + &p.pos;
                    }
                    if sim.bounce_off_walls {
                        if p.pos.x > 30.0 || p.pos.x < -30.0 {
                            p.vel.x = p.vel.x * -1.0;
                        }
                        if p.pos.y > 30.0 || p.pos.y < -30.0 {
                            p.vel.y = p.vel.y * -1.0;
                        }
                    }
                }
            }
        }
        simulator.borrow_mut().steps += count;
    }

    pub fn add_particle(&mut self, x_coord: f32, y_coord: f32, x_vel: f32, y_vel: f32) {
        self.particle_list.borrow_mut().push(Particle {
            pos: PhysVector {
                x: x_coord,
                y: y_coord,
            },
            vel: PhysVector { x: x_vel, y: y_vel },
        });
    }

    pub fn acceleration_for(&self, p: &Particle) -> PhysVector {
        let position = &p.pos;
        match self.function {
            // forces
            SimulateFunction::Gravity => PhysVector { x: 0.0, y: -20.0 },
            SimulateFunction::GravityResistive => {
                let c: f32 = -0.7;
                let a = PhysVector { x: 0.0, y: -9.8 };
                let v = &p.vel;
                &a + &(v * c)
            }
            SimulateFunction::Butterfly => {
                let theta = position.x.atan2(position.y);
                PhysVector {
                    x: theta.cos() * position.x * -1.0,
                    y: theta.sin() * position.y * -1.0,
                }
            }
            SimulateFunction::WindowsXP => {
                let pull = 2.0;
                let theta = position.y.atan2(position.x);
                PhysVector {
                    x: (theta - pull).sin() * position.x.abs(),
                    y: (theta - pull).cos() * position.y.abs() * -1.0,
                }
            }
            SimulateFunction::Logistic => {
                let k = 0.8;
                #[allow(non_snake_case)]
                let L = 15.0;
                PhysVector {
                    x: 1.0,
                    y: k * position.y * (1.0 - (position.y / L)),
                }
            }
            SimulateFunction::InverseSquare => {
                let theta = position.y.atan2(position.x);
                let rad_squared = position.x * position.x + position.y * position.y;
                let mag = 300.0/rad_squared;
                PhysVector {
                    x: -theta.cos() * mag,
                    y: -theta.sin() * mag
                }
            }
            SimulateFunction::Harmonic => {
                let k = 10.0;
                PhysVector { x: 0.0, y: -k*position.y }
            }
            _ => PhysVector { x: 0.0, y: 0.0 },
        }
    }

    pub fn time(&self) -> f32 {
        self.steps as f32 * self.timestep
    }
}

impl Particle {
    pub fn get_pos(&self) -> PhysVector {
        self.pos.clone()
    }

    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Particle {
        Particle {
            pos: PhysVector { x: x, y: y },
            vel: PhysVector { x: vx, y: vy },
        }
    }
}

impl fmt::Display for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::from(format!("Time: {:.*}\n", 4, self.time()));
        for v in self.particle_list.borrow().iter() {
            output.push_str(
                format!(
                    "[{:.*}, {:.*}], v=[{:.*}, {:.*}]\n",
                    5, v.pos.x, 5, v.pos.y, 5, v.vel.x, 5, v.vel.y
                )
                .as_str(),
            );
        }
        write!(f, "{}", output)
    }
}

use rapier2d::math::Pose2;

pub struct Weapon {
    pub weapon_type: WeaponType,
    pub local_offset: Pose2,
    pub projectile_width: f32,
    pub init_vel_mag: f32,
    pub explosion_width: Option<f32>,
    pub rpm: f32,
    pub mag_size: i8,
    pub cooldown_remaining: f32,
    
}

pub enum WeaponType {
    MachineGun,
    LaserBeam,
    MissleLauncher,
}

impl Weapon {
    pub fn fire(&mut self, dt: f32) -> bool {
        self.tick(dt);

        if self.can_fire() {
            self.cooldown_remaining = self.shot_interval();
            true
        }else {
            false
        }
        
    }

    pub fn shot_interval(&self) -> f32 {
        60.0 / self.rpm
    }

    pub fn tick(&mut self, dt: f32) {
        if self.cooldown_remaining > 0.0 {
            self.cooldown_remaining -= dt;
        }
    }

    pub fn can_fire(&self) -> bool {
        self.cooldown_remaining <= 0.0
    }

}


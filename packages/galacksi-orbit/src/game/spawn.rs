use bevy::prelude::*;
use crate::*;
use super::*;

pub const MAX_ORB_SPEED: f32 = 500.;
pub const BULLET_SPEED: f32 = 1000.;

pub struct OrbSpawner {
    pub transform: Option<Transform>,
    pub color: Option<Color>,
    pub local_player: Option<LocalPlayer>,
}

impl OrbSpawner {
    pub fn new() -> Self {
        Self {
            transform: None,
            color: None,
            local_player: None,
        }
    }

    pub fn local_player1() -> Self {
        Self {
            transform: Some(Transform::from_xyz(0., 0., 1.)),
            color: None,
            local_player: Some(LocalPlayer {
                num: 0,
                gamepad_id: None
            }),
        }
    }

    pub fn local_player(num: usize, gamepad_id: Option<usize>, local_player1_transform: &Transform) -> Self {
        /// where each flight's spacecraft is located relative to the flight leader in a finger-four formation
        const FINGER4_FORMATION_OFFSETS: [Vec3;4] = [
            Vec3::ZERO, // flight leader, front. p1
            Vec3::new(-30., -30., 0.), // flight wingman, rear-left of flight leader. p2
            Vec3::new(30., -30., 0.), // element leader, rear-right of flight leader. p3
            Vec3::new(60., -60., 0.), // element wingman, right-right of element leader. p4
        ];

        debug_assert!(num < 4, "More than 4 local players is not supported");
        let team_offset = FINGER4_FORMATION_OFFSETS[num as usize];

        let mut transform = local_player1_transform.clone();
        transform.translation += team_offset;

        Self {
            transform: Some(transform),
            color: None,
            local_player: Some(LocalPlayer {
                num,
                gamepad_id
            }),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn color_not(mut self, colors: Vec<Color>) -> Self {
        self.color = Some(Palette::rand_bloom_not(colors));
        self
    }

    fn default_color() -> Color {
        Palette::rand_bloom()
    }

    pub fn with_local_player(mut self, local_player: LocalPlayer) -> Self {
        self.local_player = Some(local_player);
        self
    }

    fn validate_fill(&mut self) -> Result<(), &'static str> {
        if self.transform.is_none() {
            return Err("Transform is not set")
        } else if self.color.is_none() {
            self.color = Some(Self::default_color());
        }

        Ok(())
    }

    pub fn spawn(
        mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Result<Self, &'static str> {
        self.validate_fill()?;

        let color = self.color.unwrap();
        let transform = self.transform.unwrap();
        let position = transform.translation.truncate();

        let mut orb = commands.spawn((
            Orb,
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(materials.add(color)),
            transform,
            Motion {
                position,
                max_speed: Some(MAX_ORB_SPEED),
                ..default()
            },
            LastPosition(position),
            EquipmentInventory([(0, InstalledEquipment::new_mounted(0, 0))].into_iter().collect()),
            OnGameScreen
        ));

        if let Some(local_player) = &self.local_player {
            if local_player.num == 0 {
                orb.insert(LocalPlayer1);
            }

            orb.insert(local_player.clone());
        }

        orb.with_children(|commands| {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(1.,1000.))),
                MeshMaterial2d(materials.add(palette::DARK_GRAY)),
                Transform::from_translation(Vec3::new(0.0, 1000./2. + 20., -0.1)),
            ));
        });

        Ok(self)
    }
}

#[derive(Default)]
pub struct BulletSpawner {
    pub transform: Option<Transform>,
    pub motion: Option<Motion>,
    pub color: Option<Color>,
}

impl BulletSpawner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_orb(orb_motion: &Motion, orb_transform: &Transform) -> Self {
        let speed = orb_motion.velocity.length() + BULLET_SPEED;
        let angle = orb_transform.rotation * Vec3::Y;
        let position = orb_motion.position.extend(1.) + angle * 40.;

        let motion = Motion {
            position: position.truncate(),
            velocity: ((angle * speed).truncate() + orb_motion.velocity),
            ..default()
        };

        let transform = Transform {
            translation: position,
            ..*orb_transform
        };

        Self {
            transform: Some(transform),
            motion: Some(motion),
            color: None,
        }
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn motion(mut self, motion: Motion) -> Self {
        self.motion = Some(motion);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    fn default_color() -> Color {
        Palette::rand_bloom()
    }

    fn validate_fill(&mut self) -> Result<(), &'static str> {
        if self.transform.is_none() {
            return Err("Transform is not set")
        } else if self.motion.is_none() {
            return Err("Motion is not set")
        } else if self.color.is_none() {
            self.color = Some(Self::default_color());
        }

        Ok(())
    }

    pub fn spawn(
        mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Result<Self, &'static str> {
        self.validate_fill()?;

        let color = self.color.unwrap();
        let transform = self.transform.unwrap();
        let motion = self.motion.as_ref().unwrap();

        commands.spawn((
            OrbBullet,
            Mesh2d(meshes.add(Circle::new(1.))),
            MeshMaterial2d(materials.add(color)),
            transform,
            motion.clone(),
            LastPosition(motion.position),
            OnGameScreen
        ));

        Ok(self)
    }
}

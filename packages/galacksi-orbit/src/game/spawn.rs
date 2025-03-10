use bevy::prelude::*;
use crate::*;
use super::*;

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
                num: 1,
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
                ..default()
            },
            LastPosition {
                position,
                ..default()
            },
            UseActions::default(),
            LastUseActions::default(),
            InstalledEquipment {
                items: [(0, EquipmentItem::new(0))]
                    .into_iter().collect()
            },
            Gear {
            items: [Some(0), None, None, None]
            },
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
                Mesh2d(meshes.add(Rectangle::new(30.,1.))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_translation(Vec3::new(0.,0.,1.)),
            ));
        });

        Ok(self)
    }
}

#[derive(Default)]
pub struct BulletSpawner {
    pub transform: Option<Transform>,
    pub speed: Option<f32>,
    pub color: Option<Color>,
}

impl BulletSpawner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_orb(motion: &Motion, transform: &Transform) -> Self {
        let speed = motion.velocity.length() + BULLET_SPEED;
        let mut transform = transform.clone();
        transform.translation = motion.position.extend(1.);

        Self {
            transform: Some(transform),
            speed: Some(speed),
            color: None,
        }
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = Some(speed);
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
        } else if self.speed.is_none() {
            return Err("Velocity is not set")
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
        let speed = self.speed.unwrap();

        let velocity = (transform.rotation * Vec3::Y * speed).truncate();
        let position = transform.translation.truncate();

        commands.spawn((
            OrbBullet,
            Mesh2d(meshes.add(Circle::new(1.))),
            MeshMaterial2d(materials.add(color)),
            transform,
            Motion {
                position,
                velocity,
                ..default()
            },
            LastPosition {
                position,
                ..default()
            },
            OnGameScreen
        ));

        Ok(self)
    }
}

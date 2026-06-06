use crate::components::velocity::Velocity;
use crate::components::sprite::Sprite;
use crate::components::{ShipComponent, ShipClass};
use crate::components::transform::Transform;
use crate::entity::{Entity,EntityType};
use crate::math::vec2math::Vec2;
use crate::assets::ship_database::ShipDatabase;

pub fn spawn_player(spaw_point:Vec2,ship_db: &ShipDatabase, class: ShipClass) -> Entity {
    let stats = ship_db.get(class);


    Entity {
        name: "Player".to_string(),
        entity_type: EntityType::Ship,

        transform: Transform {
            position: Vec2::new(spaw_point.x, spaw_point.y),
            rotation: 0.0,
            scale: Vec2::new(2.0, 2.0),
        },
        velocity: Velocity { linear: Vec2::new(0.0,0.0),angular: 0.0 },
        hitbox: stats.hitbox.clone(),
        sprite: Sprite {
            texture_id: stats.texture_id.clone(),
        },
        ship_component: Some(ShipComponent {
            class: class,
        })

    }

}

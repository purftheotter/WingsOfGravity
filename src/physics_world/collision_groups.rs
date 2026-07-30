
pub mod collision_groups {
    use rapier2d::geometry::Group;

    pub const SHIP: Group        = Group::GROUP_1;
    pub const BULLET: Group      = Group::GROUP_2;
    pub const ASTEROID_HULL: Group = Group::GROUP_3;
    pub const ASTEROID_VERT: Group = Group::GROUP_4;
}

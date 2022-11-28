pub const SCOPES: &[&str] = &[
    Activity::READ,
    Activity::WRITE,
    Gear::READ,
    Gear::WRITE,
    User::READ,
    User::WRITE,
];

pub trait Resource {
    const READ: &'static str;
    const WRITE: &'static str;
}

pub struct Activity;
pub struct Gear;
pub struct User;

impl Resource for Activity {
    const READ: &'static str = "activity:read";
    const WRITE: &'static str = "activity:write";
}

impl Resource for Gear {
    const READ: &'static str = "gear:read";
    const WRITE: &'static str = "gear:write";
}

impl Resource for User {
    const READ: &'static str = "user:read";
    const WRITE: &'static str = "user:write";
}

enum Scopes {
    ActivityRead,
    ActivityWrite,
    GearRead,
    GearWrite,
    UserRead,
    UserWrite,
}

impl std::str::FromStr for Scopes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            Activity::READ => Self::ActivityRead,
            Activity::WRITE => Self::ActivityWrite,
            Gear::READ => Self::GearRead,
            Gear::WRITE => Self::GearWrite,
            User::READ => Self::UserRead,
            User::WRITE => Self::UserWrite,
            _ => return Err(()),
        })
    }
}

pub struct Read<S>(pub S);
pub struct Write<S>(pub S);

pub trait Scope {
    const SCOPE: &'static str;
}

impl Scope for () {
    const SCOPE: &'static str = "";
}

impl<S: Resource> Scope for Read<S> {
    const SCOPE: &'static str = S::READ;
}

impl<S: Resource> Scope for Write<S> {
    const SCOPE: &'static str = S::WRITE;
}

use tf_database::{
    error::{Error, Result},
    Database, query::{GearQuery, UserQuery},
};
use tf_models::{gear::Gear, user::User, GearId, UserId};

#[test]
fn insert_gear_without_existing_owner() {
    let db = Database::create_temporary().unwrap();

    let user_id = UserId::from_bytes(nanoid::nanoid!().as_bytes()).unwrap();
    let query = GearQuery {
        user_id,
        id: GearId::from_bytes(nanoid::nanoid!().as_bytes()).unwrap(),
    };

    let gear = Gear::default();

    let actual = db
        .root()
        .unwrap()
        .traverse::<Gear>()
        .unwrap()
        .insert(&query, &gear, &UserQuery { user_id })
        .unwrap_err();

    let expected = Error::ForeignKeyConstraint;
    assert!(matches!(actual, expected));
}

#[test]
fn insert_gear_with_existing_owner() -> Result<()> {
    let db = Database::create_temporary().unwrap();

    let user_id = UserId::from_bytes(nanoid::nanoid!().as_bytes()).unwrap();

    let query = UserQuery { user_id };
    let user = User {
        name: "Test".into(),
        heartrate_rest: 50,
        heartrate_max: 205,
    };

    db.root()?.insert(&query, &user)?;

    let gear_query = GearQuery {
        user_id,
        id: GearId::from_bytes(nanoid::nanoid!().as_bytes()).unwrap(),
    };
    let gear = Gear::default();

    db.root()?
        .traverse::<Gear>()?
        .insert(&gear_query, &gear, &query)?;
    let gear: Option<Gear> = db.root()?.traverse::<Gear>()?.get(&gear_query)?;
    assert!(matches!(gear, Some(_)));

    Ok(())
}

#[test]
fn get_gear_after_deleting_owner() -> Result<()> {
    let db = Database::create_temporary()?;

    let user_id = UserId::from_bytes(nanoid::nanoid!().as_bytes()).unwrap();

    let user_query = UserQuery { user_id };
    let user = User {
        name: "Test".into(),
        heartrate_rest: 50,
        heartrate_max: 205,
    };

    db.root()?.insert(&user_query, &user)?;

    let gear_query = GearQuery {
        user_id,
        id: GearId::from_bytes(nanoid::nanoid!().as_bytes()).unwrap(),
    };
    let gear = Gear::default();

    db.root()?
        .traverse::<Gear>()?
        .insert(&gear_query, &gear, &user_query)?;

    db.root::<User>()?.remove(&user_query)?;
    assert!(matches!(db.root::<User>()?.get(&user_query)?, None));

    let actual: Option<Gear> = db.root()?.traverse::<Gear>()?.get(&gear_query)?;
    dbg!(&actual);
    assert!(matches!(actual, None));

    Ok(())
}

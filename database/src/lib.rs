use rusqlite::{params, Connection, Result};
#[derive(Debug,Clone)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}
pub fn first() -> Result<Vec<String>> {
    let conn = Connection::open("./database.db")?;

    //conn.execute(
    //    "CREATE TABLE person (
    //              id              INTEGER PRIMARY KEY,
    //              name            TEXT NOT NULL,
    //              data            BLOB
    //              )",
    //    [],
    //)?;
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;
    let mut names = Vec::new();
    for person in person_iter {
        let tttt2 :Person = person.unwrap();
        let ssss :String = String::from("{\"id\":\"")+&tttt2.clone().name+"\","+
                            "\"data\":\""+&tttt2.clone().name+"\""+
                            "}";
        //let s = 2;
        //println!("{}",ssss);
        names.push(ssss);
        //println!("Found person {:?}", person.unwrap());
    }
    Ok(names)
}
pub fn data(msg:String) -> Result<Vec<String>> {
    let conn = Connection::open("./database.db")?;

    //conn.execute(
    //    "CREATE TABLE person (
    //              id              INTEGER PRIMARY KEY,
    //              name            TEXT NOT NULL,
    //              data            BLOB
    //              )",
    //    [],
    //)?;
    let me = Person {
        id: 0,
        name: msg,
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;
    let mut names = Vec::new();
    for person in person_iter {
        let tttt2 :Person = person.unwrap();
        let ssss :String = String::from("{\"id\":\"")+&tttt2.clone().name+"\","+
                            "\"data\":\""+&tttt2.clone().name+"\""+
                            "}";
        //let s = 2;
        //println!("{}",ssss);
        names.push(ssss);
        //println!("Found person {:?}", person.unwrap());
    }
    Ok(names)
}

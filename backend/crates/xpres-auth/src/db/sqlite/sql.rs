

pub(crate) const CREATE_TABLE: &str = r#"

CREATE TABLE IF NOT EXISTS "users" (
	"id" INTEGER NOT NULL UNIQUE,
	"email" TEXT NOT NULL UNIQUE,
	"password" TEXT NOT NULL,
	"is_admin" BOOLEAN NOT NULL DEFAULT false,
	PRIMARY KEY("id"),
	FOREIGN KEY ("id") REFERENCES "ticket"("owner_id")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "routes" (
	"id" INTEGER NOT NULL UNIQUE,
	"start" TEXT NOT NULL,
	"end" TEXT NOT NULL,
	"data" BLOB,
	PRIMARY KEY("id"),
	FOREIGN KEY ("id") REFERENCES "ticket"("route_id")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "ticket" (
	"owner_id" INTEGER NOT NULL,
	"id" INTEGER NOT NULL UNIQUE,
	"route_id" INTEGER NOT NULL,
	"data" BLOB,
	PRIMARY KEY("owner_id")
);


"#;

// USER

pub(crate) const INSERT_USER: &str = "
INSERT INTO users (email, password, is_admin) VALUES (?1, ?2, ?3);
";

pub(crate) const UPDATE_USER: &str = "
UPDATE users SET 
    email = ?2,
    password = ?3,
    is_admin = ?4
WHERE
    id = ?1;
";

pub(crate) const SELECT_BY_ID: &str = "
SELECT * FROM users WHERE id = ?1;
";

pub(crate) const SELECT_BY_EMAIL: &str = "
SELECT * FROM users WHERE email = ?1;
";

pub(crate) const REMOVE_BY_ID: &str = "
DELETE FROM users WHERE id =?1;
";
pub(crate) const REMOVE_BY_EMAIL: &str = "
DELETE FROM users WHERE email =?1;
";

// ROUTES

pub(crate) const INSERT_ROUTE: &str = "
INSERT INTO routes (start, end, data) VALUES (?1, ?2, ?3);
";

pub(crate) const REMOVE_ROUTE_BY_ID: &str = "
DELETE FROM routes WHERE id = ?1;
";

pub(crate) const REMOVE_ROUTE_BY_START_END: &str = "
DELETE FROM routes WHERE start = ?1 AND end = ?2;
";

pub(crate) const SELECT_ROUTE_BY_ID: &str = "
SELECT * FROM routes WHERE id = ?1;
";

pub(crate) const SELECT_ROUTE_BY_START: &str = "
SELECT * FROM routes WHERE start = ?1;
";

// TICKET

pub(crate) const INSERT_TICKET: &str = "
INSERT INTO ticket (owner_id, route_id, data) VALUES (?1, ?2, ?3);
";

pub(crate) const REMOVE_TICKET_BY_OWNER_ID: &str = "
DELETE FROM ticket WHERE owner_id = ?1;
";

pub(crate) const REMOVE_TICKET_BY_ROUTE_ID: &str = "
DELETE FROM ticket WHERE route_id = ?1;
";

pub(crate) const SELECT_TICKET_BY_ID: &str = "
	SELECT * FROM ticket where id = ?1
";

pub(crate) const SELECT_TICKET_BY_OWNER_ID: &str = "
SELECT * FROM ticket WHERE owner_id = ?1;
";

pub(crate) const SELECT_TICKET_BY_ROUTE_ID: &str = "
SELECT * FROM ticket WHERE route_id = ?1;
";

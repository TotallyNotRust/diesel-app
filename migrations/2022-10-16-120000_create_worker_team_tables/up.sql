-- Your SQL goes here
CREATE TABLE "worker" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "team" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "team_worker" (
	"id" INTEGER NOT NULL,
	"worker_id"	INTEGER NOT NULL,
	"team_id"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("worker_id") REFERENCES "worker"("id"),
	FOREIGN KEY("team_id") REFERENCES "team"("id")
);

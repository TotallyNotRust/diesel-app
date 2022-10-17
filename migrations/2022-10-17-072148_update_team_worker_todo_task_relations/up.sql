-- Your SQL goes here
DROP TABLE "team_worker";
DROP TABLE "worker";
DROP TABLE "team";
DROP TABLE "todo";
DROP TABLE "task";

CREATE TABLE "task" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
    "team_id" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
    FOREIGN KEY ("team_id") REFERENCES "team"("id")
);

CREATE TABLE "todo" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
    "is_completed" INTEGER NOT NULL,
    "task_id" INTEGER NOT NULL,
    "worker_id" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
    FOREIGN KEY ("worker_id") REFERENCES "worker"("id")
);

CREATE TABLE "team" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
    "current_task" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
    FOREIGN KEY ("current_task") REFERENCES "task"("id")
);
CREATE TABLE "worker" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
    "current_todo" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
    FOREIGN KEY ("current_todo") REFERENCES "todo"("id")
);

CREATE TABLE "team_worker" (
	"id" INTEGER NOT NULL,
	"worker_id"	INTEGER NOT NULL,
    "team_id" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("worker_id") REFERENCES "worker"("id")
);

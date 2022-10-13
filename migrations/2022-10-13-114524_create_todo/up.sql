-- Your SQL goes here
CREATE TABLE "todo" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"todoId"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("todoId") REFERENCES "task"("id")
);
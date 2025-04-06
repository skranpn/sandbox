import sqlite3

dbname = "database.db"
conn = sqlite3.connect(dbname)

# show existing tables
cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table';")
print("Existing tables:")
for row in cursor.fetchall():
    print(row[0])

# drop users table if it exists
conn.execute("DROP TABLE IF EXISTS users;")
# create users table
conn.execute("""
CREATE TABLE users (
    name TEXT NOT NULL,
    age INTEGER NOT NULL
);
""")

# show records (from users table)
cursor = conn.execute("SELECT * FROM users;")
print("Records in users table:")
for row in cursor.fetchall():
    print(row)

# insert a new record into users table
conn.execute("INSERT INTO users (name, age) VALUES (?, ?)", ("Alice", 20))
conn.commit()

# show records (from users table)
cursor = conn.execute("SELECT * FROM users;")
print("Records in users table after insert:")
for row in cursor.fetchall():
    print(row)

conn.close()

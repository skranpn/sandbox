import os
import sqlite3
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("SQLite Explorer")

@mcp.resource("schema://main")
def get_schema() -> str:
    """Provide the database schema as a resource"""
    conn = sqlite3.connect("database.db")
    schema = conn.execute("SELECT sql FROM sqlite_master WHERE type='table'").fetchall()
    return "\n".join(sql[0] for sql in schema if sql[0])

@mcp.prompt()
def list_users_prompt() -> str:
    return f"Please giveme a list of users in the database"

@mcp.tool()
def list_users() -> str:
    """List all user names in the database"""
    dir = os.path.dirname(os.path.realpath(__file__))
    db = os.path.join(dir, "database.db")
    conn = sqlite3.connect(db)
    cursor = conn.execute("SELECT * FROM users")
    users = cursor.fetchall()
    return "\n".join(row[0] for row in users)

@mcp.tool()
def get_user(name: str) -> str:
    """Get a user by name"""
    dir = os.path.dirname(os.path.realpath(__file__))
    db = os.path.join(dir, "database.db")
    conn = sqlite3.connect(db)
    cursor = conn.execute("SELECT * FROM users WHERE name=?", (name,))
    user = cursor.fetchone()
    if user:
        return f"Name: {user[0]}, Age: {user[1]}"
    else:
        return "User not found"

if __name__ == "__main__":
    mcp.run()

import {Client} from "pg";

const pgClient = new Client("postgresql://myuser:mypassword@localhost:5432/learning")


const pgClient2 = new Client({
    user: "myuser",
    host: "localhost",
    database: "learning",
    password: "mypassword",
    port: 5432

})


async function main() {
    await pgClient.connect();
    const response = await pgClient.query("SELECT * FROM users");


console.log(response)
   
}
// https://projects.100xdevs.com/tracks/YOSAherHkqWXhOdlE4yE/sql-1
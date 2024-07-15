import { Socket } from "node:net";

const getClient = () =>
  new Promise((res, rej) => {
    const client = new Socket();

    client.connect(1256, "localhost", () => {
      res(client);
    });

    client.on("error", (err) => rej(err));
  });

const waitForData = (client) =>
  new Promise((res) => {
    let data = "";
    client.on("data", (_data) => {
      data += _data;
    });
    client.on("close", () => {
      res(data);
    });
  });

const set = async (key, value) => {
  const client = await getClient();
  client.write(`SET\r\n${key}\r\n${value}\r\n\r\n`);
  return await waitForData(client);
};

const get = async (key) => {
  const client = await getClient();
  client.write(`GET\r\n${key}\r\n\r\n`);
  return await waitForData(client);
};

const list = async () => {
  const client = await getClient();
  client.write("LIST\r\n\r\n\r\n");
  return await waitForData(client);
};

const main = async () => {
  console.log("SET key value ->", await set("key", "value"));
  console.log("GET key ->", await get("key"));
  console.log("LIST -> ", await list());
};

main();

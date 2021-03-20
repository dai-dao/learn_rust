import asyncio
from asonic import Client
from asonic.enums import Channel


async def main():
    c = Client(host='::1', port=1491, password='SecretPassword',
                max_connections=100)
    await c.channel(Channel.SEARCH)
    print(await c.query("collection", 'bucket', 'recipe'))


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
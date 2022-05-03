const qs = require('qs')
const https = require('https')
const express = require('express')
const fs = require('fs').promises

async function main() {
  const cert = await fs.readFile('localhost.pem')
  const key = await fs.readFile('localhost-key.pem')
  const app = express()

  app.get('/', (req, res) => {
    let { counter = '0' } =
      (req.headers.cookie && qs.parse(req.headers.cookie)) || {}
    console.debug('counter', counter)
    counter = Number(counter) + 1
    res.cookie('counter', counter, {
      httpOnly: true,
      maxAge: 1000 * 86400,
      path: '/',
      sameSite: 'Lax',
      secure: true
    })
    res.status(200).send(`${counter}`)
  })

  https
    .createServer({ cert, key }, app)
    .listen(3000, () => console.info('server is listening on 3000'))
}

main().then(console.debug).catch(console.error)

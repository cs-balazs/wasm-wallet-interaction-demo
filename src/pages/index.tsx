import { Button, Group, Input } from "@mantine/core"
import type { NextPage } from "next"
import { useEffect, useState } from "react"

const Home: NextPage = () => {
  const [wasm, setWasm] = useState<Promise<typeof import("../wasm/roost")>>(null)
  const [msg, setMsg] = useState<string>("")

  useEffect(() => {
    setWasm(
      import("../wasm/roost").then((module) => module.default().then(() => module))
    )
  }, [])

  const ethereumSign = async () => {
    if (!wasm) console.error("WASM is not yet initialized")
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const { ethereum_sign } = await wasm
    ethereum_sign(msg)
  }

  const polkadotSign = async () => {
    if (!wasm) console.error("WASM is not yet initialized")
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const { polkadot_sign } = await wasm
    polkadot_sign(msg)
  }

  return (
    <Group>
      <Input value={msg} onChange={(e) => setMsg(e.target.value)} />
      <Button onClick={ethereumSign}>Ethereum sign</Button>
      <Button onClick={polkadotSign}>Polkadot sign</Button>
    </Group>
  )
}

export default Home

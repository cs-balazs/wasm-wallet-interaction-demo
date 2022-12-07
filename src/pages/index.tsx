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

  const onEthereumSign = async () => {
    if (!wasm) console.error("WASM is not yet initialized")
    const { ethereumSign } = await wasm
    ethereumSign(msg)
  }

  const onPolkadotSign = async () => {
    if (!wasm) console.error("WASM is not yet initialized")
    const { polkadotSign } = await wasm
    polkadotSign(msg)
  }

  return (
    <Group>
      <Input value={msg} onChange={(e) => setMsg(e.target.value)} />
      <Button onClick={onEthereumSign}>Ethereum sign</Button>
      <Button onClick={onPolkadotSign}>Polkadot sign</Button>
    </Group>
  )
}

export default Home

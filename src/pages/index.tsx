import { Button, Group } from "@mantine/core"
import type { NextPage } from "next"
import { useEffect, useState } from "react"

const Home: NextPage = () => {
  const [wasm, setWasm] = useState<Promise<typeof import("../wasm/roost")>>(null)

  useEffect(() => {
    setWasm(
      import("../wasm/roost").then((module) => module.default().then(() => module))
    )
  }, [])

  const ethereumSign = async () => {
    if (!wasm) console.error("WASM is not yet initialized")
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const { ethereum_sign } = await wasm
    ethereum_sign("almafa 2")
  }

  const polkadotSign = async () => {
    if (!wasm) console.error("WASM is not yet initialized")
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const { polkadot_sign } = await wasm
    polkadot_sign("almafa 2")
  }

  return (
    <Group>
      <Button onClick={ethereumSign}>Ethereum sign</Button>
      <Button onClick={polkadotSign}>Polkadot sign</Button>
    </Group>
  )
}

export default Home

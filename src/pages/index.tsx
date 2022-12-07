import { Button } from "@mantine/core"
import type { NextPage } from "next"
import { useEffect, useState } from "react"
import { InitOutput } from "../wasm/roost"

const Home: NextPage = () => {
  const [wasm, setWasm] = useState<Promise<InitOutput>>(null)

  useEffect(() => {
    setWasm(
      import("../wasm/roost").then((module) => {
        console.log(module)
        return module.default()
      })
    )
  }, [])

  const tstWasm = async () => {
    if (!wasm) console.error("WASM is not yet initialized")
    const { test } = await wasm
    test()
  }

  return (
    <>
      <Button onClick={tstWasm}>Tst</Button>
    </>
  )
}

export default Home

import { Box, Button, Image } from "@mantine/core"
import { useAccount, useConnect, useEnsAvatar, useEnsName } from "wagmi"
import shortenHex from "../../../utils/shortenHex"
import { useWeb3Connection } from "../Web3ConnectionContext"

const ConnectButton = () => {
  const { address, isConnecting } = useAccount()
  const { data: ensAvatar } = useEnsAvatar({ address })
  const { data: ensName } = useEnsName({ address })
  const { error, isLoading } = useConnect()
  const { openConnectionModal, openAccountModal } = useWeb3Connection()

  return (
    <Button
      loading={isLoading || isConnecting}
      leftIcon={
        ensAvatar && (
          <Box
            sx={{ width: 24, height: 24, borderRadius: "50%", overflow: "hidden" }}
          >
            <Image src={ensAvatar} alt={ensName ?? address} width={24} height={24} />
          </Box>
        )
      }
      color={error ? "red" : undefined}
      onClick={error ? undefined : address ? openAccountModal : openConnectionModal}
    >
      {isLoading || isConnecting
        ? "Connecting"
        : error
        ? "Error"
        : address
        ? ensName ?? shortenHex(address, 3)
        : "Connect"}
    </Button>
  )
}

export default ConnectButton

ESPLORA=https://btc-signet.gobob.xyz
BOB_RPC=https://bob-sepolia.rpc.gobob.xyz/
DEPLOYER_PRIVATE_KEY=

height=$(curl -s $ESPLORA/blocks/tip/height)
echo "tip = $height"

retarget_height=$((height - height % 2016))
echo "retarget_height = $retarget_height"

block_hash=$(curl -s $ESPLORA/block-height/$retarget_height)
echo "block_hash = $block_hash"

swapped_endianness_blockhash=$(echo "$block_hash" | grep -o '..' | tac | tr -d '\n')
echo "swapped_endianness_blockhash = $swapped_endianness_blockhash"

header=$(curl -s $ESPLORA/block/$block_hash/header)
echo "header = $header"

forge create src/relay/FullRelayWithVerify.sol:FullRelayWithVerify \
    --rpc-url $BOB_RPC \
    --private-key $DEPLOYER_PRIVATE_KEY \
    --priority-gas-price 1 \
    --legacy \
    --broadcast \
    --constructor-args 0x$header $retarget_height 0x$swapped_endianness_blockhash

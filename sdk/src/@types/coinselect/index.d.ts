declare module "coinselect" {
    interface Input {
        txId: string,
        vout: number,
        value: number,
        // For use with PSBT:
        // not needed for coinSelect, but will be passed on to inputs later
        nonWitnessUtxo?: Buffer,
        // OR
        // if your utxo is a segwit output, you can use witnessUtxo instead
        witnessUtxo?: {
            script: Buffer,
            value: number,
        }
    }

    interface Output {
        address?: string,
        value: number
    }

    /**
    * Blackjack, with Accumulative fallback
    * @param inputs The UTXOs to spend from.
    * @param outputs The transaction outputs.
    * @param feeRate The current Bitcoin fee rate.
    * @returns The fee paid to the network.
    */
    function coinSelect(
        inputs: Array<Input>,
        outputs: Array<Output>,
        feeRate: number // satoshis per byte
    ): {
        inputs?: Array<Input>,
        outputs?: Array<Output>,
        fee: number,
    };

    export = coinSelect;
} 
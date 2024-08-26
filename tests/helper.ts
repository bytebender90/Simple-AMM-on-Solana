import * as anchor from '@coral-xyz/anchor';

export const readLogs = async (
    connection: anchor.web3.Connection,
    tx: string
) => {
    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
        {
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: tx,
        },
        'confirmed'
    );

    const details = await connection.getTransaction(tx, {
        maxSupportedTransactionVersion: 0,
        commitment: 'confirmed',
    });

    return details.meta.logMessages;
};

/**
 * Base path for Ordinals regtest Indexer.
 * @default "http://0.0.0.0:80"
 */
export const REGTEST_ORD_BASE_PATH = "http://0.0.0.0:80";

// ToDo: Add mainnet and testnet indexer endpoint once exposed.
/**
 * Base path for Ordinals mainnet Indexer.
 * @default ""
 */
export const MAINNET_ORD_BASE_PATH = "";

/**
 * Base path for Ordinals testnet Indexer.
 * @default ""
 */
export const TESTNET_ORD_BASE_PATH = "";


/**
 * @ignore
 */
export type InscriptionId = string & { length: 64 };

/**
 * @ignore
 */
export type InscriptionContent = string;


/**
 * Represents a collection of inscriptions along with related metadata.
 */
export interface InscriptionsData {
    /**
     * An array of inscription content.
     */
    inscriptions: InscriptionContent[];

    /**
     * The previous entry.
     */
    prev: string;

    /**
     * The next entry.
     */
    next: string;

    /**
     * The lowest value.
     */
    lowest: number;

    /**
     * The highest value.
     */
    highest: number;
}

/**
 * Represents information about an Inscription UTXO.
 */
export interface InscriptionUTXO {
    /**
     * The value of the UTXO.
     */
    value: number;

    /**
     * The script public key.
     */
    script_pubkey: string;

    /**
     * The address associated with the UTXO.
     */
    address: string;

    /**
     * The transaction related to the UTXO.
     */
    transaction: string;

    /**
     * The SAT ranges.
     */
    sat_ranges: string;

    /**
     * An array of inscription content.
     */
    inscriptions: InscriptionContent[];

    /**
     * A map of runes.
     */
    runes: Record<string, any>;
}

/**
 * Represents an ordinal.
 */
export interface Ordinal {
    /**
     * The number of the ordinal.
     */
    number: number;

    /**
     * The decimal representation of the ordinal.
     */
    decimal: string;

    /**
     * The degree representation of the ordinal.
     */
    degree: string;

    /**
     * The name of the ordinal.
     */
    name: string;

    /**
     * The block associated with the ordinal.
     */
    block: number;

    /**
     * The cycle associated with the ordinal.
     */
    cycle: number;

    /**
     * The epoch associated with the ordinal.
     */
    epoch: number;

    /**
     * The period associated with the ordinal.
     */
    period: number;

    /**
     * The offset associated with the ordinal.
     */
    offset: number;

    /**
     * The rarity of the ordinal.
     */
    rarity: string;

    /**
     * The percentile representation of the ordinal.
     */
    percentile: string;

    /**
     * The SAT point of the ordinal.
     */
    satpoint: string;

    /**
     * The timestamp of the ordinal.
     */
    timestamp: number;

    /**
     * An array of inscription content.
     */
    inscriptions: InscriptionContent[];
}

/**
 * Represents information about an inscription based on its ID.
 */
export interface InscriptionDataFromId {
    /**
     * The address associated with the inscription.
     */
    address: string;

    /**
     * An array of child IDs.
     */
    children: string[];

    /**
     * The length of the content.
     */
    content_length: number;

    /**
     * The content type of the inscription.
     */
    content_type: string;

    /**
     * The genesis fee of the inscription.
     */
    genesis_fee: number;

    /**
     * The genesis height of the inscription.
     */
    genesis_height: number;

    /**
     * The ID of the inscription.
     */
    inscription_id: string;

    /**
     * The number of the inscription.
     */
    inscription_number: number;

    /**
     * The next inscription ID.
     */
    next: string;

    /**
     * The output value of the inscription.
     */
    output_value: number;

    /**
     * The parent inscription ID.
     */
    parent: string;

    /**
     * The previous inscription ID.
     */
    previous: string;

    /**
     * The rune associated with the inscription.
     */
    rune: string;

    /**
     * The SAT associated with the inscription.
     */
    sat: string;

    /**
     * The SAT point of the inscription.
     */
    satpoint: string;

    /**
     * The timestamp of the inscription.
     */
    timestamp: number;
}


export interface OrdinalsClient {
    /**
     * Retrieves an inscription based on its ID.
     * @param {string} id - The ID of the inscription to retrieve.
     * @returns {Promise<InscriptionDataFromId>} A Promise that resolves to the inscription data.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let inscription_id: InscriptionId = "enter_your_inscription_id_here" as InscriptionId;
     * const inscriptions: InscriptionDataFromId = await client.getInscriptionFromId(id);
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptionFromId(id: InscriptionId): Promise<InscriptionDataFromId>;

    /**
     * Retrieves a list of inscriptions.
     * @returns {Promise<InscriptionsData>} A Promise that resolves to a inscriptions data.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * const inscriptions: InscriptionsData = await client.getInscriptions();
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptions(): Promise<InscriptionsData>;

    /**
     * Retrieves an inscription based on its block height.
     * @param {number} height - The block height of the inscription to retrieve.
     * @returns {Promise<InscriptionsData>} A Promise that resolves to the inscription data.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let block: number = "enter_your_block_number_here";
     * const inscriptions: InscriptionsData = await client.getInscriptionsFromBlock(block);
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptionsFromBlock(height: number): Promise<InscriptionsData>;

    /**
     * Retrieves an inscription based on its UTXO (Unspent Transaction Output).
     * @param {string} utxo - The UTXO of the inscription to retrieve.
     * @returns {Promise<InscriptionUTXO>} A Promise that resolves to the inscription data.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let utxo: string = "enter_your_utxo_here";
     * const inscriptions: InscriptionUTXO = await client.getInscriptionFromUTXO(utxo);
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptionFromUTXO(utxo: string): Promise<InscriptionUTXO>;

    /**
     * Retrieves an inscription based on its sat (something specific to your use case).
     * @param {number} sat - The sat of the inscription to retrieve.
     * @returns {Promise<Ordinal>} A Promise that resolves to the ordinal data type.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let sat: string = "enter_your_sat_number_here";
     * const ordinal: Ordinal = await client.getInscriptionsFromSat(sat);
     * console.log("Ordinal Data:", inscriptions);
     * ```
     */
    getInscriptionsFromSat(sat: number): Promise<Ordinal>;
}

export class DefaultOrdinalsClient implements OrdinalsClient {
    private basePath: string;

    constructor(networkOrUrl: string = "regtest") {
        switch (networkOrUrl) {
            case "mainnet":
                this.basePath = MAINNET_ORD_BASE_PATH;
                break;
            case "testnet":
                this.basePath = TESTNET_ORD_BASE_PATH;
                break;
            case "regtest":
                this.basePath = REGTEST_ORD_BASE_PATH;
                break;
            default:
                this.basePath = networkOrUrl;
        }
    }

    /**
     * @ignore
     */
    async getInscriptionFromId(id: InscriptionId): Promise<InscriptionDataFromId> {
        return await this.getJson<InscriptionDataFromId>(`${this.basePath}/inscription/${id}`);
    }


    /**
     * @ignore
     */
    async getInscriptions(): Promise<InscriptionsData> {
        return await this.getJson<InscriptionsData>(`${this.basePath}/inscriptions`);
    }

    /**
     * @ignore
     */
    async getInscriptionsFromBlock(height: number): Promise<InscriptionsData> {
        return await this.getJson<InscriptionsData>(`${this.basePath}/inscriptions/block/${height}`);
    }

    /**
     * @ignore
     */
    async getInscriptionFromUTXO(utxo: string): Promise<InscriptionUTXO> {
        return await this.getJson<InscriptionUTXO>(`${this.basePath}/output/${utxo}`);
    }

    /**
     * @ignore
     */
    async getInscriptionsFromSat(sat: number): Promise<Ordinal> {
        return await this.getJson<Ordinal>(`${this.basePath}/sat/${sat}`);
    }

    /**
     * @ignore
     */
    async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url, {
            headers: {
                'Accept': 'application/json',
            },
        });
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.json() as Promise<T>;
    }
}

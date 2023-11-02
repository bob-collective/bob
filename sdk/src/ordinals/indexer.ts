import {MAINNET_ESPLORA_BASE_PATH, TESTNET_ESPLORA_BASE_PATH} from "../electrs";

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
 * @ignore
 */
export interface InscriptionsData {
    inscriptions: InscriptionContent[];
    prev: string;
    next: string;
    lowest: number;
    highest: number;
}

/**
 * @ignore
 */
export interface InscriptionUTXO {
    value: number;
    script_pubkey: string,
    address:string,
    transaction:string,
    sat_ranges: string,
    inscriptions: InscriptionContent[];
}

/**
 * @ignore
 */
export interface Ordinal {
    decimal: string,
    degree: string,
    name: string,
    block: number,
    cycle: number,
    epoch: number,
    period: number,
    offset: number,
    rarity: string,
    percentile: string,
    satpoint: string,
    timestamp: number,
    inscriptions: InscriptionContent[];
}

/**
 * @ignore
 */
export interface InscriptionDataFromId {
    children: string[];
    content_length: string,
    content_type: string,
    genesis_fee: number,
    genesis_height: number,
    inscription_id: string,
    inscription_number: number,
    next: string,
    output_value: string,
    parent: string,
    previous: string,
    rune: string,
    sat: string,
    satpoint: string
    timestamp: number
}

export interface OrdinalsClient {
    /**
     * Retrieves an inscription based on its ID.
     * @param {string} id - The ID of the inscription to retrieve.
     * @returns {Promise<string>} A Promise that resolves to the inscription as a string.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let id: string = "enter_your_inscription_id_here";
     * const inscriptions: InscriptionDataFromId = await client.getInscriptionFromId(id);
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptionFromId(id: InscriptionId): Promise<InscriptionDataFromId>;

    /**
     * Retrieves a list of inscriptions.
     * @returns {Promise<string>} A Promise that resolves to a string representing a list of inscriptions.
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
     * @returns {Promise<string>} A Promise that resolves to the inscription as a string.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let block: string = "enter_your_block_number_here";
     * const inscriptions: InscriptionsData = await client.getInscriptionsFromBlock(block);
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptionsFromBlock(height: number): Promise<InscriptionsData>;

    /**
     * Retrieves an inscription based on its UTXO (Unspent Transaction Output).
     * @param {string} utxo - The UTXO of the inscription to retrieve.
     * @returns {Promise<string>} A Promise that resolves to the inscription as a string.
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
     * @returns {Promise<string>} A Promise that resolves to the inscription as a string.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let sat: string = "enter_your_sat_number_here";
     * const inscriptions: Ordinal = await client.getInscriptionsFromSat(sat);
     * console.log("Inscriptions Data:", inscriptions);
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

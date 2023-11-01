/**
 * Base path for Ordinals regtest Indexer.
 * @default "http://localhost:3003"
 */
export const REGTEST_ORD_BASE_PATH = "http://0.0.0.0:80";


/**
 * @ignore
 */
export interface InscriptionsData {
    inscriptions: string[];
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
    inscriptions: string[];
}

/**
 * @ignore
 */
export interface InscriptionSat {
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
    inscriptions: string[];
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
    getInscriptionFromId(id: string): Promise<InscriptionDataFromId>;

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
     * @param {string} height - The block height of the inscription to retrieve.
     * @returns {Promise<string>} A Promise that resolves to the inscription as a string.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let block: string = "enter_your_block_number_here";
     * const inscriptions: InscriptionsData = await client.getInscriptionFromBlock(block);
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptionFromBlock(height: string): Promise<InscriptionsData>;

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
     * @param {string} sat - The sat of the inscription to retrieve.
     * @returns {Promise<string>} A Promise that resolves to the inscription as a string.
     *
     * @example
     * ```typescript
     * const client = new DefaultOrdinalsClient("regtest");
     * let sat: string = "enter_your_sat_number_here";
     * const inscriptions: InscriptionSat = await client.getInscriptionFromSat(sat);
     * console.log("Inscriptions Data:", inscriptions);
     * ```
     */
    getInscriptionFromSat(sat: string): Promise<InscriptionSat>;
}

export class DefaultOrdinalsClient implements OrdinalsClient {
    private basePath: string;

    constructor(networkOrUrl: string = "regtest") {
        switch (networkOrUrl) {
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
    async getInscriptionFromId(id: string): Promise<InscriptionDataFromId> {
        const response = await this.getJson<{
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
        }>(`${this.basePath}/inscription/${id}`);
        return {
            children: response.children,
            content_length: response.content_length,
            content_type: response.content_type,
            genesis_fee: response.genesis_fee,
            genesis_height: response.genesis_height,
            inscription_id: response.inscription_id,
            inscription_number: response.inscription_number,
            next: response.next,
            output_value: response.output_value,
            parent: response.parent,
            previous: response.previous,
            rune: response.rune,
            sat: response.sat,
            satpoint: response.satpoint,
            timestamp: response.timestamp
        };
    }


    /**
     * @ignore
     */
    async getInscriptions(): Promise<InscriptionsData> {
        const response = await this.getJson<{
            "inscriptions": string[],
            "prev": string,
            "next": string,
            "lowest": number,
            "highest": number
        }>(`${this.basePath}/inscriptions`);
        return {
            inscriptions: response.inscriptions,
            prev: response.prev,
            next: response.next,
            lowest: response.lowest,
            highest: response.highest,
        };
    }

    /**
     * @ignore
     */
    async getInscriptionFromBlock(height: string): Promise<InscriptionsData> {
        const response = await this.getJson<{
            "inscriptions": string[],
            "prev": string,
            "next": string,
            "lowest": number,
            "highest": number
        }>(`${this.basePath}/inscriptions/block/${height}`);
        return {
            inscriptions: response.inscriptions,
            prev: response.prev,
            next: response.next,
            lowest: response.lowest,
            highest: response.highest,
        };
    }

    /**
     * @ignore
     */
    async getInscriptionFromUTXO(utxo: string): Promise<InscriptionUTXO> {
        const response = await this.getJson<{
            "value": number;
            "script_pubkey": string,
            "address":string,
            "transaction":string,
            "sat_ranges": string,
            "inscriptions": string[];
        }>(`${this.basePath}/output/${utxo}`);
        return {
            value: response.value,
            script_pubkey: response.script_pubkey,
            address: response.address,
            transaction: response.transaction,
            sat_ranges: response.sat_ranges,
            inscriptions: response.inscriptions,
        };
    }

    /**
     * @ignore
     */
    async getInscriptionFromSat(sat: string): Promise<InscriptionSat> {
        const response = await this.getJson<{
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
            inscriptions: string[];
        }>(`${this.basePath}/sat/${sat}`);
        return {
            decimal: response.decimal,
            degree: response.degree,
            name: response.name,
            block: response.block,
            cycle: response.cycle,
            epoch: response.epoch,
            period: response.period,
            offset: response.offset,
            rarity: response.rarity,
            percentile: response.percentile,
            satpoint: response.satpoint,
            timestamp: response.timestamp,
            inscriptions: response.inscriptions,
        };
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


    /**
     * @ignore
     */
    async getText(url: string): Promise<string> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.text();
    }
}

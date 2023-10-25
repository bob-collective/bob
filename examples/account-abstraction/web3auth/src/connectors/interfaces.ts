import type { IWeb3Auth, WALLET_ADAPTER_TYPE } from "@web3auth/base";
import type { IWeb3AuthModal, ModalConfig } from "@web3auth/modal";
import type { OpenloginLoginParams } from "@web3auth/openlogin-adapter";

export interface Options {
  web3AuthInstance: IWeb3Auth | IWeb3AuthModal;
  loginParams?: OpenloginLoginParams;
  modalConfig?: Record<WALLET_ADAPTER_TYPE, ModalConfig>;
}

# RainbowDAO-Protocol-Ink-milestone_1
## Contract introduction
- Kernel: controls the most basic modules, route_manage, authority_management, and role_manage of Rainbow Dao.
  At the same time, it is the administrator of route_manage contract, authority_management contract and role_manage contract.
  The above three contracts can only be called through the kernel.
  Later, the control of the kernel will be transferred to community governance, and this contract can be invoked only through governance voting.
- role_manage:It controls the role of the whole rainbow protocol. It can give roles to various controls, modules and addresses, and let different modules drive different powers.
- authority_management:Set various permissions and bind them to roles. Multiple permissions can be added to a role.
- route_manage:Used to store the name and address of each contract bound. When a contract needs to call other contracts, you can get the address here. When a contract changes its address, you only need to update the contract address in route_manage, and other contracts do not need to upgrade the code.
- users_manage:It is used to store user information using rainbow protocol.
- multisig:It is used to generate multi sign addresses, which can be used as Dao administrators. You can set the number of multiple signatures and the number of multiple signatures you need to agree to perform the operation.
- govnance_dao:It is the governance basis of the whole rainbow agreement, where you can initiate proposals to sort out the whole rainbow agreement.
- income_category:The classification of the whole rainbow agreement revenue is recorded here. When the switch of a classification is turned on, it means that he can charge.
- erc20Factory: It is a contract to generate tokens. The creator of Dao can easily generate erc20 tokens by passing in the basic information of tokens




## Installing

Please make sure that you have these prerequisites installed on your computer:

```bash
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain stable
```

Then you have to install ink! command line utility which will make setting up Substrate smart contract projects easier:

```bash
cargo install cargo-contract --vers 0.15.0 --force --locked
```

You also need the [binaryen](https://github.com/WebAssembly/binaryen) package installed on your computer which is used to optimize the WebAssembly bytecode of the contract, you can use npm to install it:

```bash
npm install -g binaryen
```

## Testing

First of all you need to clone the repository, run:

```bash
git clone https://github.com/RainbowDAO/RainbowDAO-Protocol-ink-Phase-1.git
cd RainbowDAO-Protocol-ink-Phase-1
```

Then, You can enter any folder and enter the following command.

```bash
cargo +nightly test
```

## Building

To build the WASM of your contract and metadata, You can enter any folder and enter the following command.
```bash
cargo +nightly contract build
```

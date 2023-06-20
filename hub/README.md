#Hub Contract
This is the official repo of the Hub contract built on Cosmos with CosmWasm. This contract encapsulates the concept of a Hub on the Burnt Labs Exodvs dApp. The Hub contract is responsible for managing the lifecycle of the Hub, including creating and managing the Hub's members, and managing the Hub's assets.
The Hub smart contract is designed to create a community for creators, enabling them to manage their followers, share content, sell NFTs, and more. This README provides an overview of the different states and functionalities of the Hub contract.

## States

### Config

- `owner`: Represents the address of the contract owner.

### ContractVersion

- `contract`: A globally unique identifier for the contract, following standard namespacing conventions. It helps identify the specific implementation of the contract.
- `version`: Represents the version of the contract. It can be a simple counter, a semantic version, or a custom feature flag list.

### SocialLinks

- `name`: The name of a social link associated with the Hub.
- `url`: The URL or address of the social link.

### HubMetadata

- `name`: The name of the Hub.
- `hub_url`: The URL or address of the Hub.
- `description`: A brief description of the Hub.
- `tags`: A list of tags associated with the Hub.
- `social_links`: A list of social links associated with the Hub (represented by `SocialLinks` objects).
- `creator`: The creator's name.
- `thumbnail_image_url`: The URL or address of the thumbnail image for the Hub.
- `banner_image_url`: The URL or address of the banner image for the Hub.
- `seat_contract`: An optional address of a seat contract associated with the Hub.

### MetadataField (enum)

- `SeatContract(String)`: Represents the seat contract field used for updating the seat contract associated with the Hub.

## Functionality

The Hub contract provides the following functionalities:

### Instantiate

The `instantiate` function is used to initialize the contract and its modules. It takes the following parameters:

- `ownable`: Contains the configuration for the `Ownable` module.
- `metadata`: Contains the initial metadata for the Hub.

### Execute

The `execute` function is responsible for executing various actions within the contract. It handles the following messages:

- `Ownable(msg)`: Executes actions related to the `Ownable` module.
- `UpdateMetadata(meta_field)`: Updates the metadata fields of the Hub, such as the seat contract.

### Query

The `query` function is used to query the state of the contract. It supports the following queries:

- `Ownable(query_msg)`: Retrieves information related to the `Ownable` module.
- `Metadata(query_msg)`: Retrieves information related to the Hub's metadata.

### Building the Contract
Refer to the workspace README for instructions on how to build the contract.

---

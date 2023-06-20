# Seat Contract

The Seat contract is responsible for the creation, purchase, re-sell, and other actions of ownership/membership (a.k.a seat) on a creator Hub. Creators create seats for their Hubs, and these seats are used to recognize who belongs to a particular hub. Followers can purchase seats for a particular hub to identify as part of the creator's hub. The seats are created as NFTs by this contract, which is responsible for their creation, purchase, and other actions.

## Contract Structure

The seat contract is implemented using multiple modules:

- **Ownable Module**: Handles the ownership functionality of the contract.
- **Metadata Module**: Manages the metadata of the seats, including their name, image URI, description, benefits, and other properties.
- **Seat Token Module**: Implements the functionality of the seat tokens, including minting, transferring, and querying token-related information.
- **Redeemable Module**: Handles the redemption functionality for specific items associated with the seats.
- **Sellable Module**: Implements the functionality for selling the seats.
- **Sales Module**: Manages the primary sales of the seats.

## Usage

### Instantiate

During the instantiation of the contract, the following parameters are required:

- `ownable`: Configuration for the ownable module.
- `metadata`: Configuration for the metadata module.
- `seat_token`: Configuration for the seat token module.
- `redeemable`: Configuration for the redeemable module.
- `sellable`: Optional configuration for the sellable module.
- `sales`: Configuration for the sales module.
- `hub_contract`: The address of the creator Hub contract.

### Execute Messages

The contract supports the following execute messages:

- **Ownable**: Executes operations related to ownership management.
- **Metadata**: Executes operations related to metadata management.
- **SeatToken**: Executes operations related to seat tokens.
- **Redeemable**: Executes operations related to redeemable items.
- **Sellable**: Executes operations related to selling seats.
- **Sales**: Executes operations related to seat sales.

### Query Messages

The contract supports the following query messages:

- **Ownable**: Queries ownership-related information.
- **Metadata**: Queries metadata-related information.
- **SeatToken**: Queries seat token-related information.
- **Redeemable**: Queries redeemable-related information.
- **Sellable**: Queries sellable-related information.
- **Sales**: Queries sales-related information.
- **AllSeats**: Retrieves information about all the seats.

## Error Handling

The contract defines its own set of error types that can occur during contract execution. These error types include standard errors as well as errors specific to each module.

### Building the Contract
Refer to the workspace README for instructions on how to build the contract.

---
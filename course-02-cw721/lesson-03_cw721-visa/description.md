# Title
Create the Visa Contract

# Description 
We've learned about the CW721 spec and some of the details of the `cw721-base` contract. Now, we'll move on to create our own contract, `cw721-visa`, so that we can travel (somewhat) freely between the Exodite planets.

# Coverage
- `InstantiateMsg`
- `CONFIG`
- `VisaMetadata`
- `instantiate`
    - Save `Config`
    - Create base `InstantiateMsg`
    - Send msg to default()
    - CW2 set contract version
- `execute` and `query`
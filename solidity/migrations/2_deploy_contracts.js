// We're using javascript until we have a better way to do this... I'm sorry.
const TheMostAmazingContract = artifacts.require("theMostAmazingContract");

module.exports = function (deployer) {
  deployer.deploy(TheMostAmazingContract);
};

// Going to need to setup ganache to test but kinda want to see if I can make a really really dumb version in rust. I will almost certainly give up on that idea first things tomorrow March 12, 24'

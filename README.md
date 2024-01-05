<img src="https://i.imgur.com/hA5WEnT.jpg">
<div align="center">
  <img src="https://badgen.net/badge/version/2.0/blue">
  <img src="https://badgen.net/badge/docs/2.0/blue">
  <img src="https://badgen.net/badge/contributions/open/blue">
</div>
<h1 align="center">Stockpile v2</h1>
<div align="center">
  <h3>Funding Without Barriers.</h3>
  <div align="center">
    <a href="https://stockpile.pro">Website</a>
    •
    <a href="https://twitter.com/GoStockpile">Twitter</a>
    •
    <a href="https://twitter.com/GoStockpile">LinkedIn</a>
  </div>
</div>
<br>
<h2>Overview</h2>
<p>
  Stockpile is a decentralized funding engine for the open-internet. Leveraging the speed of Solana, the protocol facilitates 
  general crowdfunding campaigns, quadratic funding rounds, and fair hackathons with verifiable transfers/calculations, all at
  the speed of light. Stockpile is designed for builders and creators to leverage their communities to raise the funds needed
  for their next endeavor, and is (to our knowledge) the first of its kind on any blockchain.
</p>
<h2>Highlights</h2>
<ul>
  <li>
    <b>
      General Crowdfunding:
    </b>
    Inspired by the likes of Kickstarter and Indiegogo, the protocol features general crowdfunding by default, which is applicable
    for most projects and initiatives. This also features optional reward distribution via NFTs, which is handled on the frontend.
  </li>
  <li>
    <b>
      Funding Rounds:
    </b>
    Stockpile features permissionless, configurable quadratic funding rounds hosted on-chain. Anyone with size is permitted to create
    a round to equitably fund projects and causes pertaining to the initiative their looking to move forward.
  </li>
  <li>
    <b>
      Milestones:
    </b>
    Most ecosystems structure grants in milestones. We've taken this into account, and implemented this on-chain in conjunction with both
    general crowdfunding, and funding rounds. Milestones are created under a project, and require the user to specify a condition, along
    with an intended completion date. In addition, future pools will be have the option to gate based on a presence of these accounts.
    As of right now, there is no approval process, and these exist only for transparency.
    *NOTE:* This feature is still experimental
  </li>
</ul>
<h2>Getting Started</h2>
<p>Ensure the Solana-CLI & Anchor-CLI are installed</p>

```
solana --version && anchor --version
```
<br>
<p>Clone this repository</p>
    
```
git clone https://github.com/StockpileProtocol/stockpile-v2.git
```
<br>
<p>Build & generate an IDL</p>
    
```
anchor build
```
<h2>Program Addresses</h2>
<p>
  Mainnet & Devnet: <a href="https://solana.fm/address/STKUaKniasuqrfer3XNbmrrc578pkL1XACdK8H3YPu8?cluster=mainnet-alpha">STKUaKniasuqrfer3XNbmrrc578pkL1XACdK8H3YPu8</a>
</p>
<h2>Disclaimer</h2>
<p>
  This code is unaudited. Copy and use at your own risk. We incur no liability in the event that a third-party uses this code, and has
  issues deriving from exploits or any other attacks.
</p>
<h2>Credits</h2>
<p>
  Special thank you to Buffalo Joe (Solana Labs) & Valentin Madrid (Squads) for helping to get us started, and Sean (Squads) for technical advisory. 
</p>


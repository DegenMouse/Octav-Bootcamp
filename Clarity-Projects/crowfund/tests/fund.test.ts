import { describe, expect, it } from "vitest";

const accounts = simnet.getAccounts();
const address1 = accounts.get("wallet_1")!;
const address2 = accounts.get("wallet_2")!;
import { Cl, cvToValue } from '@stacks/transactions';
/*
  The test below is an example. To learn more, read the testing documentation here:
  https://docs.hiro.so/stacks/clarinet-js-sdk
*/

//TODO: Fix all tests

describe("tests", () => {
  it("ensures simnet is well initalised", () => {
    expect(simnet.blockHeight).toBeDefined();
  });

  it("test create campaign", () => {
    const goal = 1000;
    const deadline = simnet.blockHeight + 100;
    const title = "Test Campaign";
    const campaign = simnet.callPublicFn('fund', 'new-campaign', [Cl.uint(goal), Cl.uint(deadline), Cl.stringAscii(title)], address1);
    expect(campaign.result).toBeOk(Cl.uint(0));
  });

  it("test contribution", () => {
    
    // Arrange
    const goal = 1000;
    const deadline = simnet.blockHeight + 100;
    const title = "Test Campaign";
    const campaign = simnet.callPublicFn('fund', 'new-campaign', [Cl.uint(goal), Cl.uint(deadline), Cl.stringAscii(title)], address1);
    const campaignId = 0;
    const amount = 100;

    // Act
    const {result: actual} = simnet.callPublicFn('fund', 'contribute', [Cl.uint(campaignId), Cl.uint(amount)], address1);

    // Assert
    expect(actual).toBeOk(Cl.tuple({
      campaign: Cl.tuple({
        id: Cl.uint(campaignId),
        owner: Cl.principal(address1),
        goal: Cl.uint(goal),
        deadline: Cl.uint(deadline),
        contributions: Cl.uint(amount)
      }),
      refund: Cl.tuple({
        amount: Cl.uint(amount),
        contributor: Cl.principal(address1),
      })
    }));
  });

  it("test claim", () => {
    // Arrange
    const goal = 1000;
    const deadline = simnet.blockHeight + 100;
    const title = "Test Campaign";
    const campaign = simnet.callPublicFn('fund', 'new-campaign', [Cl.uint(goal), Cl.uint(deadline), Cl.stringAscii(title)], address1);
    const campaignId = 0;
    const amount = 1000;
    simnet.callPublicFn('fund', 'contribute', [Cl.uint(campaignId), Cl.uint(amount)], address1);
    // simnet.mineEmptyBlocks(101);

    // Act
    const {result: actual} = simnet.callPublicFn('fund', 'claim', [Cl.uint(campaignId)], address1);

    // Assert
    expect(actual).toBeOk(Cl.bool(true));

  });

  it("test refund", () => {
    // Arrange
    const goal = 1000;
    const deadline = simnet.blockHeight + 100;
    const title = "Test Campaign";
    const campaign = simnet.callPublicFn('fund', 'new-campaign', [Cl.uint(goal), Cl.uint(deadline), Cl.stringAscii(title)], address1);
    const campaignId = 0;
    const amount = 100;
    simnet.callPublicFn('fund', 'contribute', [Cl.uint(campaignId), Cl.uint(amount)], address2);
    simnet.mineEmptyBlocks(101);

    // Act
    const {result: actual} = simnet.callPublicFn('fund', 'refund', [Cl.uint(campaignId)], address2);

    // Assert
    expect(actual).toBeOk(Cl.bool(true));
  });

});

module.exports = ({ wallets, refs, config, client }) => ({
  getGoldenR: () => client.query("clicker", { get_golden_r: {} }),
  getScores: () => client.query("clicker", {get_scores: {} }),
  upsertScore: (score, signer = wallets.validator) =>
    client.execute(signer, "clicker", { upsert_score: { score }}),
});

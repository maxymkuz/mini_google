SELECT out_website_id, in_weight, out_weight, rank FROM weight
JOIN previous_ranks ON weight.out_website_id = previous_ranks.website_id
WHERE weight.in_website_id = 1;


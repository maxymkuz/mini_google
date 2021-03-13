from transformers import pipeline, RobertaForMaskedLM, RobertaTokenizer

# Щоб це заранити треба мати тензорфлов і пайторч, тому не раньне, але
# хай буде тут на те майбутнє, в якому ми не юзатимемо еластіку чи чогось такого

model = RobertaForMaskedLM.from_pretrained("ukr-roberta-base")
tokenizer = RobertaTokenizer.from_pretrained("ukr-roberta-base")

fill_mask = pipeline("fill-mask", model=model, tokenizer=tokenizer)
fill_mask("Тарас Шевченко – великий український <mask>.")

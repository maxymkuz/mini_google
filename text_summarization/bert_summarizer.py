import torch
print(torch.cuda.memory_summary(device=None, abbreviated=False))
torch.cuda.empty_cache()

from summarizer import Summarizer
body = 'Text body that you want to summarize with BERT'
model = Summarizer()
result = model(body, ratio=0.2)  # Specified with ratio
# result = model(body, num_sentences=3)  # Will return 3 sentences

print(result)


# import torch
#
# print(torch.cuda.is_available())

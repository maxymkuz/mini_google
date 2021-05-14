with open("out.txt") as file:
    title = ""
    website = ""
    pagerank_list = []
    elastic_list = []
    for idx, line in enumerate(file):
        if idx % 3 == 0:
            title = line
        elif idx % 3 == 1:
            elastic_list.append(line.strip().split()[0] + "\n")
            elastic_list.append(title)
            pagerank_list.append(line)
        else:
            elastic_list.append(line)
        # print(idx, line[:100])

    with open("pagerank_database/collected_text_and_new_links.txt", "w") as pagerank:
        pagerank.write("".join(pagerank_list))

    with open("database_backend/data/collected.txt", "w") as elastic:
        elastic.write("".join(elastic_list))



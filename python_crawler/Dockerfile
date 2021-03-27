FROM python:latest

WORKDIR /usr/src/app

COPY . .

RUN pip install --no-cache-dir -r requirements.txt

CMD ["python", "./crawler.py", "websites.txt", "2", "4"]





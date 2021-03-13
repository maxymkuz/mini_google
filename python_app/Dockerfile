FROM python:latest
WORKDIR /code
ADD requirements.txt requirements.txt
RUN pip install -r requirements.txt
COPY db_manipulator.py db_manipulator.py
CMD ["python", "-u", "db_manipulator.py"]
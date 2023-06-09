# echo ssl setting into pg_hba.conf configuration file
echo 'hostssl all all all cert clientcert=verify-full' > /home/postgres/pgdata/data/pg_hba.conf

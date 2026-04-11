

## InfluxDB3


http://192.168.4.96:8181
Token: apiv3_DmbOocJhG3Nv1a4CbsX9ZukT-7Q9oAmErEF_dDkivsmvBQudK3AR7CTupO4cpofGyv6561kJAXTkQLjelmPKyg


Enter your choice (1-3): 1

Starting InfluxDB (Quick Start)
├─ Node ID: node0
├─ Storage: /home/tao/.influxdb/data
├─ Plugins: /home/tao/.influxdb/plugins
├─ Logs: /home/tao/.influxdb/logs/20260411_122147.log
└─ Command:
    influxdb3 serve \
     --node-id=node0 \
     --http-bind=0.0.0.0:8181 \
     --object-store=file --data-dir /home/tao/.influxdb/data --plugin-dir /home/tao/.influxdb/plugins


✓ InfluxDB 3 Core is now installed and running on port 8181. Nice!

Next Steps
├─ Run source '/home/tao/.zshrc', then access InfluxDB with influxdb3 command.
├─ Create admin token: influxdb3 create token --admin
└─ Begin writing data! Learn more at https://docs.influxdata.com/influxdb3/core/get-started/write/

┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Looking to use a UI for querying, plugins, management, and more?                       │
│ Get InfluxDB 3 Explorer at https://docs.influxdata.com/influxdb3/explorer/#quick-start │
└────────────────────────────────────────────────────────────────────────────────────────┘

     

ew token created successfully!

Token: apiv3_DmbOocJhG3Nv1a4CbsX9ZukT-7Q9oAmErEF_dDkivsmvBQudK3AR7CTupO4cpofGyv6561kJAXTkQLjelmPKyg
HTTP Requests Header: Authorization: Bearer apiv3_DmbOocJhG3Nv1a4CbsX9ZukT-7Q9oAmErEF_dDkivsmvBQudK3AR7CTupO4cpofGyv6561kJAXTkQLjelmPKyg

IMPORTANT: Store this token securely, as it will not be shown again.



### ACCESS Token



2026-04-09T01:47:24.482038Z [warning  ] 'return' in a 'finally' block
Token: apiv3_DmbOocJhG3Nv1a4CbsX9ZukT-7Q9oAmErEF_dDkivsmvBQudK3AR7CTupO4cpofGyv6561kJAXTkQLjelmPKyg
HTTP Requests Header: Authorization: Bearer apiv3_DmbOocJhG3Nv1a4CbsX9ZukT-7Q9oAmErEF_dDkivsmvBQudK3AR7CTupO4cpofGyv6561kJAXTkQLjelmPKyg


### Explorer UI 

http://192.168.4.96:8888/

## Airflow


http://192.168.4.96:8080/


{"admin": "7axYGrKUcPPycmNC"}

standalone | Starting Airflow Standalone
Simple auth manager | Password for user 'admin': yxQPnmZQXGb5scn2
standalone | Checking database is initialized



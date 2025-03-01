## Defi-Relay Webhook Listener Bot 



### Listening and Polling 

This bot listens for webhooks from DefiRelay to learn about payments.   When it learns of a new payment, it will ACK this to the DefiRelay api backend.   


In case any incoming webhooks fail, this listener will also periodically poll the DefiRelay api backend for 'un-acked' payment triggers , ACk them, and handle them .


### Handling   

When this bot learns of a payment, it will add a payment record to the Postgresql Database running in Supabase to ultimately update the status of a user.  


### Forking 
If you fork this bot, you can do anything you would like!  Maybe you use a different database, maybe you have some other way of setting a users record as having paid.  This is just an example.     






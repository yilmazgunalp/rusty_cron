cron job configuration

1. create a crontab file
2. append cron entry to existing crontab
3. replace crontab file
4. add support for human readable 

examples:

#add a crontab file
$> rusty_cron add cronjobs.txt

#add a cronjob
$> rusty_cron append "0 0 1 * *" "/usr/bin/run_some.sh"

#replace a crontab file
$> rusty_cron replace newcronjobs.txt

#add cronjon in readable format
$> rusty_cron append "everyday at 6am" "~/back-up.sh" 

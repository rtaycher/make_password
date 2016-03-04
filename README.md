# make_password

generate memorable and/or easily transcribable passwords including

1. xkcd horse battery correct staple style passwords
2. Interspersed passwords

![Password Style](http://imgs.xkcd.com/comics/password_strength.png "To anyone who understands information theory and security and is in an infuriating argument with someone who does not (possibly involving mixed case), I sincerely apologize.")

Most of the time I'm satisfied with the random
passwords generated and saved in keepass, but sometimes
I need passwords I can manually re-enter easily if not
necessarily remember. 

run `make_password -n x` to change the # of words used (default to 5)

run `make_password -a` to add a random a random char followed by a random symbol as an extra word in a random position

ex:

`make_password -n 3 -a` 

might return:

`triumph syndicating 6@ sacrificial`

alternatively you can use this to mix 2 phrases for an easily memorable password.

Of course it doesn't come up with the phrases but it saves from having to write out the results yourself.

Especially usefull for re-generating the actual password for cmdline entry(where you can't move left and right).

Running 

    make_password mixed 'a bee CD' 'Dog Party? 2017!'

results in `aDobgeePaCrDty?2017!`

If anyone has any other usefull password generation patterns I'd be willing to add them.
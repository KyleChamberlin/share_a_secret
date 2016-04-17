# How to contribute

Third-party patches are an essential part of open source software. We want
to keep it as easy as possible to contribute changes that get things working.
There are a few guidelines that we need contributors to follow so that we can
have a chance of keeping on top of things.

## Getting Started

* Make sure you have a [GitHub account](https://github.com/signup/free)
* Submit an issue on GitHub, assuming one does not already exist.
  * Clearly describe the issue including steps to reproduce when it is a bug.
  * Make sure you fill in the earliest version that you know has the issue.
* Fork the repository on GitHub

## Making Changes

* Create a topic branch from where you want to base your work.
  * This is usually the master branch.
  * Only target release branches if you are certain your fix must be on that
    branch.
  * To quickly create a topic branch based on master; `git checkout -b
    fix/master/my_contribution master`. Please avoid working directly on the
    `master` branch.
* Make commits of logical units.
* Check for unnecessary whitespace with `git diff --check` before committing.
* Make sure your commit messages are in the proper format.

```
    (ISSUE NUMBER) Title of the bug being fixed.

    Without this patch applied the bug persists.  This is a problem because the
    contributor is left to imagine what the commit message should look like
    based on a description rather than an example.  This patch fixes the
    problem by making the example concrete and imperative.

    The first line should be a real life issue number from the GitHub issue tracker.
    The body describes the behaviour without the patch, why this is a problem, and
    how the patch fixes the problem when applied.
```

* Make sure you have added the necessary tests for your changes.
* Run _all_ the tests to assure nothing else was accidentally broken.

## Making Trivial Changes

### Documentation

For changes of a trivial nature to comments and documentation, it is not
always necessary to create a new issue on GitHub. In this case, it is
appropriate to start the first line of a commit with '(doc)' instead of
an issue number.

```
    (doc) Add typo/documentation commit example to CONTRIBUTING

    There is no example for contributing a documentation commit
    to the Puppet repository. This is a problem because the contributor
    is left to assume how a commit of this nature may appear.

    The first line is a real life imperative statement with '(doc)' in
    place of what would have been the ticket number in a
    non-documentation related commit. The body describes the nature of
    the new documentation or comments added.
```

## Submitting Changes

* Sign the [Contributor License Agreement](https://www.clahub.com/agreements/KyleChamberlin/mail-manager-python-interface).
* Push your changes to a topic branch in your fork of the repository.
* Submit a pull request to the repository in the organization.
* The core team looks at Pull Requests on a regular basis.
* After feedback has been given we expect responses within two weeks. After two
  weeks will may close the pull request if it isn't showing any activity.

# Additional Resources

* [Contributor License Agreement](https://www.clahub.com/agreements/KyleChamberlin/mail-manager-python-interface)
* [General GitHub documentation](http://help.github.com/)
* [GitHub pull request documentation](http://help.github.com/send-pull-requests/)
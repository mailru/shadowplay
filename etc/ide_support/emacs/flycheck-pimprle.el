;; flycheck-pimprle.el --- Flycheck binding to pimprle -*- lexical-binding: t; -*-

;; Copyright (C) 2021-2022 Evgenii Lepikhin
;; Copyright (C) 2021-2022 VK Company
;;
;; Author: Evgenii Lepikhin <johnlepikhin@gmail.com>
;; Maintainer: Evgenii Lepikhin <johnlepikhin@gmail.com>

;;; Commentary:

;;; Code:

(eval-when-compile
  (require 'pcase)          ; `pcase-dolist' (`pcase' itself is autoloaded)
  )

(defun flycheck-parse-pimprle-lint (output checker buffer)
  "Parse JSON OUTPUT of CHECKER on BUFFER as Pimprle errors."
  (mapcar (lambda (err)
            (let-alist err
              (flycheck-error-new-at
               .location.line
               .location.column
               (pcase .error_type
                 ("ManifestSyntax" 'error)
                 ("ManifestLint" 'warning)
                 ("Hiera" 'warning)
                 ("YAML" 'warning)
                 (_ 'error))
               .message
               :checker checker
               :buffer buffer
               :filename .location.path)))
          (flycheck-parse-json output)))

(flycheck-define-checker puppet-pimprle
  "A Puppet DSL linter using pimperle."
  :command ("pimprle" "check" "-f" "json" "pp" source)
  :error-parser flycheck-parse-pimprle-lint
  :modes puppet-mode)

(add-to-list 'flycheck-checkers 'puppet-pimprle)

(provide 'flycheck-pimprle)

;;; flycheck-pimprle.el ends here

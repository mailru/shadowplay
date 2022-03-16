;; flycheck-shadowplay.el --- Flycheck binding to shadowplay -*- lexical-binding: t; -*-

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

(defun flycheck-parse-shadowplay-lint (output checker buffer)
  "Parse JSON OUTPUT of CHECKER on BUFFER as Shadowplay errors."
  (mapcar (lambda (err)
            (let-alist err
              (flycheck-error-new
               :line .range.start.line
               :column .range.start.column
               :level (pcase .error_type
                 ("ManifestSyntax" 'error)
                 ("ManifestLint" 'warning)
                 ("Hiera" 'warning)
                 ("YAML" 'warning)
                 (_ 'error))
               :message .message
               :end-line (and .range.end.line .range.end.line)
               :end-column (and .range.end.column (+ .range.end.column 1))
               :checker checker
               :buffer buffer
               :filename .range.path)))
          (flycheck-parse-json output)))

(flycheck-define-checker puppet-shadowplay
  "A Puppet DSL linter using pimperle."
  :command ("shadowplay" "check" "-f" "json" "pp" source)
  :error-parser flycheck-parse-shadowplay-lint
  :modes puppet-mode)

(add-to-list 'flycheck-checkers 'puppet-shadowplay)

(provide 'flycheck-shadowplay)

;;; flycheck-shadowplay.el ends here

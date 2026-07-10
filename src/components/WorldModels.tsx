"use client";

import ScrollReveal from "./ScrollReveal";
import SealStamp from "./SealStamp";
import { useApp } from "@/context/AppContext";

function ProjectLink({ href, label }: { href: string; label: string }) {
  return (
    <a
      href={href}
      target="_blank"
      rel="noopener noreferrer"
      className="inline-flex items-center gap-1.5 text-sm text-warm/55 hover:text-seal transition-colors"
    >
      {label}
      <span aria-hidden="true">↗</span>
    </a>
  );
}

export default function WorldModels() {
  const { t } = useApp();

  return (
    <section id="world-models" className="relative py-32 px-6 md:px-8 paper-texture scroll-mt-20">
      <div className="divider-ink mb-32" />

      <div className="relative z-10 max-w-6xl mx-auto">
        <ScrollReveal className="text-center mb-20">
          <span className="text-base tracking-[0.3em] uppercase text-seal/60 block mb-5">
            {t.research.label}
          </span>
          <h2 className="font-serif text-4xl md:text-5xl text-warm font-bold mb-8">
            {t.research.title}
          </h2>
          <p className="text-warm/60 max-w-3xl mx-auto text-xl font-light leading-relaxed">
            {t.research.subtitle}
          </p>
        </ScrollReveal>

        <div className="grid lg:grid-cols-2 gap-6">
          {t.research.projects.map((project, index) => (
            <ScrollReveal key={project.name} delay={index * 0.12}>
              <article className="feature-card stamp-frame h-full relative overflow-hidden bg-surface/[0.025] p-8 md:p-10">
                <span className="absolute top-5 right-5 font-serif text-sm tracking-[0.18em] text-warm/25">
                  {String(index + 1).padStart(2, "0")}
                </span>

                <div className="flex items-start gap-5 mb-7">
                  <SealStamp text={project.seal} size="lg" />
                  <div className="pt-1">
                    <p className="text-xs uppercase tracking-[0.2em] text-seal/60 mb-2">
                      {project.kind}
                    </p>
                    <h3 className="font-serif text-3xl text-warm font-bold">
                      {project.name}
                    </h3>
                  </div>
                </div>

                <p className="text-warm/60 font-light leading-relaxed text-lg min-h-[7.5rem]">
                  {project.description}
                </p>

                <div className="mt-8 pt-6 border-t border-surface/[0.07] flex flex-wrap gap-x-6 gap-y-3">
                  <ProjectLink href={project.projectUrl} label={t.research.projectPage} />
                  <ProjectLink href={project.githubUrl} label={t.research.code} />
                  <ProjectLink href={project.artifactUrl} label={t.research.artifacts} />
                </div>
              </article>
            </ScrollReveal>
          ))}
        </div>

        <ScrollReveal delay={0.2} className="mt-8">
          <div className="rounded-2xl border border-gold/15 bg-gold/[0.035] px-7 py-7 md:px-10 md:py-8 flex flex-col lg:flex-row lg:items-center gap-6 lg:gap-10">
            <div className="shrink-0">
              <p className="text-xs uppercase tracking-[0.2em] text-gold/65 mb-2">
                {t.research.loopLabel}
              </p>
              <h3 className="font-serif text-2xl text-warm font-bold">
                {t.research.loopTitle}
              </h3>
            </div>
            <div className="hidden lg:block w-px self-stretch bg-gold/15" />
            <p className="text-warm/55 font-light leading-relaxed">
              {t.research.loopDescription}
            </p>
          </div>
        </ScrollReveal>

        <ScrollReveal delay={0.25} className="mt-10 flex flex-col sm:flex-row items-center justify-center gap-4">
          <a
            href="https://github.com/NU-World-Model-Embodied-AI"
            target="_blank"
            rel="noopener noreferrer"
            className="w-full sm:w-auto text-center px-6 py-3 rounded-xl border border-seal/35 text-seal hover:bg-seal hover:text-white transition-all"
          >
            {t.research.exploreGithub} ↗
          </a>
          <a
            href="https://huggingface.co/NU-World-Model-Embodied-AI"
            target="_blank"
            rel="noopener noreferrer"
            className="w-full sm:w-auto text-center px-6 py-3 rounded-xl border border-gold/30 text-gold/85 hover:bg-gold hover:text-ink transition-all"
          >
            {t.research.exploreHuggingFace} ↗
          </a>
        </ScrollReveal>
      </div>
    </section>
  );
}

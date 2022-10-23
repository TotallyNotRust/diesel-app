using System;
using System.Collections.Generic;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata;

namespace Opgave_2._8
{
    public partial class databaseContext : DbContext
    {
        public databaseContext()
        {
        }

        public databaseContext(DbContextOptions<databaseContext> options)
            : base(options)
        {
        }

        public virtual DbSet<Group> Groups { get; set; } = null!;
        public virtual DbSet<GroupUser> GroupUsers { get; set; } = null!;
        public virtual DbSet<User> Users { get; set; } = null!;

        protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
        {
          /*  if (!optionsBuilder.IsConfigured)
            {*/
                optionsBuilder.UseSqlite("Data Source=../../../database.db");
            Console.WriteLine(Directory.GetCurrentDirectory());
        }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            modelBuilder.Entity<Group>(entity =>
            {
                entity.Property(e => e.Name).HasColumnType("TEXT(32)");
            });

            modelBuilder.Entity<GroupUser>(entity =>
            {
                entity.HasKey(e => new { e.GroupId, e.UserId });

                entity.ToTable("GroupUser");

                entity.Property(e => e.GroupId).HasColumnType("INT");

                entity.Property(e => e.UserId).HasColumnType("INT");
            });

            modelBuilder.Entity<User>(entity =>
            {
                entity.ToTable("User");

                entity.Property(e => e.Password).HasColumnType("CHAR(36)");

                entity.Property(e => e.Username).HasColumnType("VARCHAR(32)");
            });

            OnModelCreatingPartial(modelBuilder);
        }

        partial void OnModelCreatingPartial(ModelBuilder modelBuilder);
    }
}
